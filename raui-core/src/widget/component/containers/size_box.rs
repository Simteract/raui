use crate::{
    unpack_named_slots, widget,
    widget::{
        unit::size::{SizeBoxNode, SizeBoxSizeValue},
        utils::{Rect, Transform},
    },
    widget_component,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SizeBoxProps {
    #[serde(default)]
    pub width: SizeBoxSizeValue,
    #[serde(default)]
    pub height: SizeBoxSizeValue,
    #[serde(default)]
    pub margin: Rect,
    #[serde(default)]
    pub transform: Transform,
}
implement_props_data!(SizeBoxProps);

widget_component! {
    pub size_box(id, props, named_slots) {
        unpack_named_slots!(named_slots => content);
        let SizeBoxProps { width, height, margin, transform } = props.read_cloned_or_default();

        widget! {{{
            SizeBoxNode {
                id: id.to_owned(),
                props: props.clone(),
                slot: Box::new(content),
                width,
                height,
                margin,
                transform,
            }
        }}}
    }
}
