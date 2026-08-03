"""
主控制器 — 状态机调度入口。
按编号启动 task: ros2 run task_controller main_controller_node --ros-args -p task_id:=1
"""

import threading
import rclpy
from rclpy.node import Node

from task_controller.state_machine import TaskState
from task_controller.task1_line_follow import Task1LineFollow


class MainController(Node):
    """主控节点：加载并运行指定 task"""

    TASK_REGISTRY = {
        1: Task1LineFollow,
        # 2: Task2Xxx,   ← 后续扩展
        # 3: Task3Xxx,
    }

    def __init__(self):
        super().__init__("main_controller")

        self.declare_parameter("task_id", 1)
        task_id = self.get_parameter("task_id").value
        self.get_logger().info(f"MainController 启动, task_id={task_id}")

        task_cls = self.TASK_REGISTRY.get(task_id)
        if task_cls is None:
            self.get_logger().fatal(f"未知 task_id={task_id}, 可用: {list(self.TASK_REGISTRY)}")
            raise ValueError(f"Unknown task_id: {task_id}")

        self.task = task_cls(self)
        self._task_done = False
        self._shutting_down = False

        self.task.start()

        # 主循环定时器 (10Hz 检查完成条件)
        self._check_timer = self.create_timer(0.1, self._check_loop)

    @property
    def task_done(self) -> bool:
        return self._task_done

    def _check_loop(self):
        """主循环：每 0.1s 检查任务是否完成"""
        if not self.task.is_active or self._shutting_down:
            return

        self.task.on_update()

        if self.task.check_done():
            self._on_task_done()

    def _on_task_done(self):
        """任务完成后的处理 → 打印结果 → 干净退出"""
        self._shutting_down = True
        self.task.stop()
        self._check_timer.cancel()

        self.get_logger().info("=" * 50)
        self.get_logger().info(f"  任务[{self.task.name}] 执行完成!")
        if self.task.result_msg:
            self.get_logger().info(f"  结果: {self.task.result_msg}")
        self.get_logger().info("=" * 50)

        # 从独立线程调用 shutdown，避免死锁
        threading.Thread(target=self._delayed_shutdown, daemon=True).start()

    def _delayed_shutdown(self):
        self.get_logger().info("正在退出...")
        rclpy.shutdown()

    def destroy_node(self):
        if self.task and self.task.is_active:
            self.task.stop()
        super().destroy_node()


def main():
    rclpy.init()
    node = MainController()

    try:
        rclpy.spin(node)
    except (KeyboardInterrupt, rclpy.executors.ExternalShutdownException):
        pass
    finally:
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()
