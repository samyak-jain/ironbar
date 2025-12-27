use super::{CustomWidget, CustomWidgetContext, WidgetConfig};
use crate::build;
use crate::gtk_helpers::IronbarContainer;
use crate::modules::custom::r#box::ModuleAlignment;
use gtk::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "extras", derive(schemars::JsonSchema))]
pub struct OverlayWidget {
    /// Widget name.
    ///
    /// **Default**: `null`
    name: Option<String>,

    /// Widget class name.
    ///
    /// **Default**: `null`
    class: Option<String>,

    /// Horizontal alignment of the overlay relative to its parent.
    ///
    /// **Valid options**: `start`, `center`, `end`, `fill`
    /// **Default**: `fill`
    halign: Option<ModuleAlignment>,

    /// Vertical alignment of the overlay relative to its parent.
    ///
    /// **Valid options**: `start`, `center`, `end`, `fill`
    /// **Default**: `fill`
    valign: Option<ModuleAlignment>,

    /// The child widget that is at the base of this overlay.
    /// Any overlays you add are stacked on top of this widget.
    /// This is a required field.
    child: Box<WidgetConfig>,

    /// Widgets to add to this overlay.
    /// The first widget is the base (determines sizing),
    /// subsequent widgets are overlays stacked on top.
    ///
    /// **Default**: `[]`
    overlays: Vec<OverlayLayer>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "extras", derive(schemars::JsonSchema))]
pub struct OverlayLayer {
    /// The widget or module to add as an overlay layer.
    #[serde(flatten)]
    widget: WidgetConfig,

    /// Horizontal alignment of this overlay layer within the overlay.
    ///
    /// **Valid options**: `start`, `center`, `end`, `fill`
    /// **Default**: `fill`
    halign: Option<ModuleAlignment>,

    /// Vertical alignment of this overlay layer within the overlay.
    ///
    /// **Valid options**: `start`, `center`, `end`, `fill`
    /// **Default**: `fill`
    valign: Option<ModuleAlignment>,

    /// Whether mouse events should pass through this overlay to widgets below.
    /// Set to `false` for interactive overlays (buttons/controls).
    ///
    /// **Default**: `true`
    pass_through: Option<bool>,
}

impl CustomWidget for OverlayWidget {
    type Widget = gtk::Overlay;

    fn into_widget(self, context: CustomWidgetContext) -> Self::Widget {
        let overlay = build!(self, Self::Widget);

        if let Some(halign) = self.halign {
            overlay.set_halign(halign.into());
        }

        if let Some(valign) = self.valign {
            overlay.set_valign(valign.into());
        }

        self.child.widget.add_to(
            IronbarContainer::Overlay(&overlay),
            &context,
            self.child.common,
        );

        for overlay_item in self.overlays {
            let overlay_passthrough = overlay_item.pass_through.unwrap_or(true);
            if let Some(widget) = overlay_item.widget.widget.add_to(
                IronbarContainer::Overlay(&overlay),
                &context,
                overlay_item.widget.common,
            ) {
                if let Some(halign) = overlay_item.halign {
                    widget.set_halign(halign.into());
                }
                if let Some(valign) = overlay_item.valign {
                    widget.set_valign(valign.into());
                }
                if overlay_passthrough {
                    widget.set_can_target(false);
                }
            }
        }

        overlay
    }
}
