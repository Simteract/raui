use crate::ui::components::{
    app::{AppMessage, AppSharedProps},
    inventory::InventoryMessage,
};
use raui_core::prelude::*;
use raui_material::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemCellProps {
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub thin: bool,
}
implement_props_data!(ItemCellProps, "ItemCellProps");

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub index: usize,
}
implement_props_data!(ItemData, "ItemData");

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemCellsProps {
    pub items: Vec<ItemCellProps>,
}
implement_props_data!(ItemCellsProps, "ItemCellsProps");

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OwningInventoryProps(pub WidgetId);
implement_props_data!(OwningInventoryProps, "OwningInventoryProps");

widget_hook! {
    use_item_cell(life_cycle) {
        life_cycle.change(|context| {
            for msg in context.messenger.messages {
                if let Some(msg) = msg.downcast_ref::<ButtonMessage>() {
                    if msg.action == ButtonAction::TriggerStart {
                        match msg.sender.key() {
                            "prev" => {
                                let id = context
                                    .shared_props
                                    .read_cloned_or_default::<OwningInventoryProps>()
                                    .0;
                                context.messenger.write(id, InventoryMessage::Prev);
                            }
                            "next" => {
                                let id = context
                                    .shared_props
                                    .read_cloned_or_default::<OwningInventoryProps>()
                                    .0;
                                context.messenger.write(id, InventoryMessage::Next);
                            }
                            _ => {
                                if let Ok(data) = context.props.read::<ItemData>() {
                                    let id = context
                                        .shared_props
                                        .read_cloned_or_default::<AppSharedProps>()
                                        .0;
                                    context.messenger.write(id, AppMessage::ShowPopup(data.index));
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

widget_component! {
    pub item_cell(id, key, props) [use_item_cell] {
        let ItemCellProps { image, thin } = props.read_cloned_or_default();
        let button_props = props.clone().with(SizeBoxProps {
            width: SizeBoxSizeValue::Exact(if thin { 18.0 } else { 24.0 }),
            height: SizeBoxSizeValue::Exact(24.0),
            margin: Rect {
                left: if thin { -4.0 } else { 1.0 },
                right: if thin { -4.0 } else { 1.0 },
                top: 2.0,
                bottom: 2.0,
            },
        }).with(ButtonSettingsProps {
            notify: Some(id.to_owned()),
            ..Default::default()
        });
        let panel_props = props.clone().with(PaperProps {
            variant: "cell".to_owned(),
            frame: None,
        });
        let component = if thin { content_box } else { paper };

        if image.is_empty() {
            widget! {
                (#{key} button: {button_props} {
                    content = (#{"panel"} component: {panel_props})
                })
            }
        } else {
            let image_props = Props::new(ImageBoxProps {
                content_keep_aspect_ratio: Some(ImageBoxAspectRatio {
                    horizontal_alignment: 0.5,
                    vertical_alignment: 0.5,
                }),
                material: ImageBoxMaterial::Image(ImageBoxImage {
                    id: image,
                    ..Default::default()
                }),
                ..Default::default()
            }).with(ContentBoxItemLayout {
                margin: Rect {
                    left: 4.0,
                    right: 4.0,
                    top: 4.0,
                    bottom: 4.0,
                },
                ..Default::default()
            });

            widget! {
                (#{key} button: {button_props} {
                    content = (#{"panel"} component: {panel_props} [
                        (#{"icon"} image_box: {image_props})
                    ])
                })
            }
        }
    }
}