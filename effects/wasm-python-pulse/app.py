"""wasm-python-pulse: a WebAssembly xfetch effect written in Python.

The effect reveals the content line by line and ends with a growing block
cursor, settling exactly on the original lines. It is compiled to a component
with `componentize-py` (world `effect`) and reads the `EffectRequest` JSON
directly, like any other effect binary.

Python's own thread-based budgets are unavailable on wasm; the host enforces
the manifest deadline through epoch interruption instead.
"""

import json

from wit_world.imports import host

BLOCKS = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"]


class WitWorld:
    """Effect entrypoint: JSON request in, JSON frames out."""

    def run(self, request: str) -> str:
        payload = json.loads(request)
        lines = payload.get("lines", [])
        args = payload.get("args") or {}

        duration_ms = max(1, int(args.get("duration_ms") or 800))
        fps = max(1, int(args.get("fps") or 30))
        frame_count = max(1, duration_ms * fps // 1000)
        delay = max(1, 1000 // fps)

        host.log("info", f"wasm-python-pulse: {frame_count + 1} frames")

        frames = []
        for frame in range(frame_count + 1):
            progress = frame / frame_count
            shown = min(len(lines), int(progress * len(lines) + 0.999))
            block = BLOCKS[min(len(BLOCKS) - 1, int(progress * len(BLOCKS)))]
            frame_lines = list(lines[:shown])
            if shown < len(lines):
                frame_lines.append(block)
            frames.append(
                {
                    "delay_ms": 1 if frame == frame_count else delay,
                    "lines": frame_lines or [block],
                }
            )

        return json.dumps({"frames": frames})
