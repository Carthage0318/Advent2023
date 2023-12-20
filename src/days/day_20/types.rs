#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub(super) enum PulseType {
    #[default]
    Low,
    High,
}

#[derive(Debug, Copy, Clone)]
pub(super) struct SentPulse {
    pub(super) pulse_type: PulseType,
    pub(super) destination: NodeOutput,
}

#[derive(Debug, Copy, Clone)]
pub(super) struct NodeOutput {
    pub(super) to_node: usize,
    pub(super) input_id: usize,
}

impl NodeOutput {
    pub(super) fn as_sent_pulse(self, pulse_type: PulseType) -> SentPulse {
        SentPulse {
            pulse_type,
            destination: self,
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct FlipFlop {
    pub(super) outputs: Vec<NodeOutput>,
    pub(super) is_on: bool,
    pub(super) input_count: usize,
}

#[derive(Debug, Default)]
pub(super) struct Conjunction {
    pub(super) outputs: Vec<NodeOutput>,
    pub(super) cached_inputs: Vec<PulseType>,
}

#[derive(Debug, Default)]
pub(super) struct Broadcast {
    pub(super) outputs: Vec<NodeOutput>,
    pub(super) input_count: usize,
}

#[derive(Debug, Default)]
pub(super) struct Untyped {
    pub(super) input_count: usize,
}

#[derive(Debug)]
pub(super) enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction), // What's your function?
    Broadcast(Broadcast),
    Untyped(Untyped),
}

impl Module {
    pub(super) fn new_flip_flop() -> Self {
        Self::FlipFlop(FlipFlop::default())
    }

    pub(super) fn new_conjunction() -> Self {
        Self::Conjunction(Conjunction::default())
    }

    pub(super) fn new_broadcast() -> Self {
        Self::Broadcast(Broadcast::default())
    }

    pub(super) fn new_untyped() -> Self {
        Self::Untyped(Untyped::default())
    }
}
