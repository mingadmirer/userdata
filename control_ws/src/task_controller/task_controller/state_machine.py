"""
状态机基类 — 所有 task 继承此类。
提供统一的运行/停止/状态切换接口。
"""

from enum import Enum
import rclpy
from rclpy.node import Node


class TaskState(Enum):
    IDLE = "IDLE"
    RUNNING = "RUNNING"
    PAUSED = "PAUSED"
    COMPLETED = "COMPLETED"
    FAILED = "FAILED"


class BaseTask:
    """任务基类。

    子类需实现:
      - on_enter():   进入任务时调用一次
      - on_update():  每周期调用 (由 spin 或 timer 驱动)
      - on_exit():    退出任务时调用一次
      - check_done(): 返回 True 表示任务完成
    """

    def __init__(self, node: Node, name: str = "base"):
        self.node = node
        self.name = name
        self.state = TaskState.IDLE
        self._result_msg = ""

    # ── 生命周期 ──────────────────────────────────────────
    def start(self):
        if self.state != TaskState.IDLE:
            self.node.get_logger().warn(f"[{self.name}] cannot start from {self.state.value}")
            return
        self.state = TaskState.RUNNING
        self.node.get_logger().info(f"[{self.name}] → RUNNING")
        self.on_enter()

    def stop(self):
        if self.state in (TaskState.IDLE, TaskState.COMPLETED, TaskState.FAILED):
            return
        self.state = TaskState.COMPLETED
        self.node.get_logger().info(f"[{self.name}] → COMPLETED")
        self.on_exit()

    def fail(self, reason: str = ""):
        self.state = TaskState.FAILED
        self._result_msg = reason
        self.node.get_logger().error(f"[{self.name}] → FAILED: {reason}")
        self.on_exit()

    # ── 子类接口 ──────────────────────────────────────────
    def on_enter(self):
        """进入任务，初始化资源"""

    def on_update(self):
        """每周期执行一次的主逻辑"""

    def on_exit(self):
        """退出任务，清理资源"""

    def check_done(self) -> bool:
        """返回 True 表示任务目标达成，会自动调用 stop()"""
        return False

    @property
    def is_active(self) -> bool:
        return self.state == TaskState.RUNNING

    @property
    def result_msg(self) -> str:
        return self._result_msg
