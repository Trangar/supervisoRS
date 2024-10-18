use rustc_hash::FxHashMap;

use crate::{Cardinal, FluidId, InOutput, ItemId, ItemOrFluidId, Node, NodeId};

pub fn demo_nodes() -> FxHashMap<NodeId, Node> {
    let mut nodes = FxHashMap::default();
    nodes.insert(
        NodeId(0),
        Node {
            id: NodeId(0),
            x: 100.0,
            y: 100.0,
            inputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(0)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(0)),
                    rate: 1.0,
                },
            ],
            outputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(1)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(1)),
                    rate: 1.0,
                },
            ],

            direction: Cardinal::East,
        },
    );
    nodes.insert(
        NodeId(1),
        Node {
            id: NodeId(1),
            x: 200.0,
            y: 200.0,

            inputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(1)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(1)),
                    rate: 1.0,
                },
            ],
            outputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(2)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(2)),
                    rate: 1.0,
                },
            ],

            direction: Cardinal::South,
        },
    );
    nodes.insert(
        NodeId(2),
        Node {
            id: NodeId(2),
            x: 300.0,
            y: 300.0,

            inputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(2)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(2)),
                    rate: 1.0,
                },
            ],
            outputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(3)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(3)),
                    rate: 1.0,
                },
            ],

            direction: Cardinal::West,
        },
    );
    nodes.insert(
        NodeId(3),
        Node {
            id: NodeId(3),
            x: 400.0,
            y: 400.0,

            inputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(3)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(3)),
                    rate: 1.0,
                },
            ],
            outputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(0)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(0)),
                    rate: 1.0,
                },
            ],

            direction: Cardinal::North,
        },
    );
    nodes
}
