from __future__ import annotations
from typing import List
import heapq


class Solution:
    def getOrder(self, tasks: List[List[int]]) -> List[int]:
        next_tasks: list[tuple[int, int]] = []
        tasks_processing_order: list[int] = []

        sorted_tasks = [
            (enqueue, process, idx) for idx, (enqueue, process) in enumerate(tasks)
        ]
        sorted_tasks.sort()

        curr_time = 0
        task_index = 0

        while task_index < len(tasks) or next_tasks:
            if not next_tasks and curr_time < sorted_tasks[task_index][0]:
                curr_time = sorted_tasks[task_index][0]

            while (
                task_index < len(sorted_tasks)
                and curr_time >= sorted_tasks[task_index][0]
            ):
                _, process_time, original_index = sorted_tasks[task_index]
                heapq.heappush(next_tasks, (process_time, original_index))
                task_index += 1

            process_time, index = heapq.heappop(next_tasks)

            curr_time += process_time
            tasks_processing_order.append(index)

        return tasks_processing_order


def main():
    inputs: list[list[list[int]]] = [
        [[1, 2], [2, 4], [3, 2], [4, 1]],
        [[7, 10], [7, 12], [7, 5], [7, 4], [7, 2]],
    ]

    solution = Solution()
    for tasks in inputs:
        result = solution.getOrder(tasks)
        print(result)


if __name__ == "__main__":
    main()
