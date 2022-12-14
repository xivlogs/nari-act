"""Parse director commands from ACT log lines"""
from typing import Optional

from nari.types import Timestamp
from nari.types.event import Event
from nari.types.event.instance import BarrierState, BarrierToggle
from nari.types.event.instance import InstanceComplete, InstanceFade, InstanceInit, Fade
from nari.types.director import DirectorUpdateCommand


def director_events_from_logline(timestamp: Timestamp, params: list[str]) -> Optional[Event]: # pylint: disable=too-many-return-statements
    """Parses a director command event from an ACT log line

    ACT Event ID (decimal): 33

    ## Param layout from ACT

    The first two params in every event is the ACT event ID and the timestamp it was parsed; the following table documents all the other fields.

    This event will be one of `BarrierToggle`, `InstanceComplete`, `InstanceVote`, `InstanceFade`, or `InstanceInit`.

    |Index|Type|Description|
    |----:|----|:----------|
    |0    |int|The first two bytes are from the category, and the second two bytes make up the instance ID.|
    |1    |int|The director command ID.|
    |2-N  ||Depends on the command.|
    """
    instance_id = int(params[0][:4], 16)
    command = int(params[1], 16)

    match command:
        case DirectorUpdateCommand.barrierup:
            return BarrierToggle(
                timestamp=timestamp,
                instance_id=instance_id,
                state=BarrierState.up,
            )
        case DirectorUpdateCommand.barrierdown:
            return BarrierToggle(
                timestamp=timestamp,
                instance_id=instance_id,
                state=BarrierState.down,
            )
        case DirectorUpdateCommand.complete:
            return InstanceComplete(
                timestamp=timestamp,
                instance_id=instance_id,
            )
        case DirectorUpdateCommand.fadein:
            return InstanceFade(
                timestamp=timestamp,
                instance_id=instance_id,
                state=Fade.In,
            )
        case DirectorUpdateCommand.fadeout:
            return InstanceFade(
                timestamp=timestamp,
                instance_id=instance_id,
                state=Fade.Out,
            )
        case DirectorUpdateCommand.init:
            return InstanceInit(
                timestamp=timestamp,
                instance_id=instance_id,
            )
        case _:
            return None
