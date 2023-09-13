from __future__ import annotations

from typing import TYPE_CHECKING

import numpy as np

if TYPE_CHECKING:
    from . import Quaternion, Rotation3DLike, RotationAxisAngle


class Rotation3DExt:
    @staticmethod
    def inner__field_converter_override(
        data: Rotation3DLike,
    ) -> Quaternion | RotationAxisAngle:
        from . import Quaternion, Rotation3D, RotationAxisAngle

        if isinstance(data, Rotation3D):
            return data.inner
        elif isinstance(data, (Quaternion, RotationAxisAngle)):
            return data
        else:
            return Quaternion(xyzw=np.array(data))
