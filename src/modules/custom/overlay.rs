use super::{CustomWidget, CustomWidgetContext, WidgetConfig};
use crate::build;
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

    /// Widgets to add to this overlay.
    /// The first widget is the base (determines sizing),
    /// subsequent widgets are overlays stacked on top.
    ///
    /// **Default**: `[]`
    widgets: Vec<OverlayLayer>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(feature = "extras", derive(schemars::JsonSchema))]
pub struct OverlayLayer {
    /// The widget or module to add as an overlay layer.
    #[serde(flatten)]
    widget: WidgetConfig,

    /// Horizontal alignment of this overlay layer.
    ///
    /// **Valid options**: `start`, `center`, `end`, `fill`
    /// **Default**: `center`
    #[serde(default = "default_layer_halign")]
    halign: ModuleAlignment,

    /// Vertical alignment of this overlay layer.
    ///
    /// **Valid options**: `start`, `center`, `end`, `fill`
    /// **Default**: `center`
    #[serde(default = "default_layer_valign")]
    valign: ModuleAlignment,

    /// Whether mouse events should pass through this overlay to widgets below.
    /// Set to `false` for interactive overlays (buttons/controls).
    ///
    /// Note: This feature requires GTK 4.14+ and is currently not available.
    ///
    /// **Default**: `true`
    #[serde(default = "default_pass_through")]
    #[allow(dead_code)]
    pass_through: bool,

    /// Whether this overlay should affect the size of the container.
    ///
    /// Note: This feature requires GTK 4.14+ and is currently not available.
    ///
    /// **Default**: `false`
    #[serde(default)]
    #[allow(dead_code)]
    measure: bool,
}

fn default_layer_halign() -> ModuleAlignment {
    ModuleAlignment::Center
}

fn default_layer_valign() -> ModuleAlignment {
    ModuleAlignment::Center
}

fn default_pass_through() -> bool {
    true
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

        // Process widgets: first is the base child, rest are overlays
        let mut widgets = self.widgets.into_iter();

        // Set the base child (first widget)
        if let Some(first_layer) = widgets.next() {
            let base_widget = create_layer_widget(&first_layer, &context, true);
            overlay.set_child(Some(&base_widget));
        }

        // Add remaining widgets as overlays
        for layer in widgets {
            let layer_widget = create_layer_widget(&layer, &context, false);

            // Apply alignment to the layer widget
            layer_widget.set_halign(layer.halign.into());
            layer_widget.set_valign(layer.valign.into());

            // Configure overlay-specific properties
            overlay.add_overlay(&layer_widget);

            // Note: set_overlay_pass_through and set_measure_overlay may not be available
            // in GTK 4.12. These methods were added in later versions of GTK4.
            // For now, overlays will always pass through events (default GTK behavior)
            // and won't affect container measurements.
        }

        overlay
    }
}

/// Creates a widget for an overlay layer.
/// This handles both custom widgets and native modules.
fn create_layer_widget(
    layer: &OverlayLayer,
    context: &CustomWidgetContext,
    _is_base: bool,
) -> gtk::Widget {
    // For the base layer, we add the widget directly to get proper sizing
    // For overlay layers, we also add them directly but with overlay properties

    // We need to create a temporary container to capture the widget
    let temp_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    layer
        .widget
        .clone()
        .widget
        .add_to(&temp_container, context, layer.widget.common.clone());

    // Get the first (and should be only) child from the temp container
    temp_container
        .first_child()
        .expect("Layer widget should have created a child")
}
