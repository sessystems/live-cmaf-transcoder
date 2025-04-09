use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct VideoFilterDeinterlace {
    pub enable: bool,
}

impl VideoFilterDeinterlace {
    pub fn new() -> Self {
        Self { enable: false }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq,ToSchema)]
pub enum TextPosition {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Clone, Deserialize, Serialize, Debug, ToSchema)]
pub struct VideoFilterDrawText {
    pub enabled: bool,
    pub text: String,
    pub fontsize: u32,
    pub color: String,
    pub position: TextPosition,
}

impl VideoFilterDrawText {
    pub fn new() -> Self {
        Self {
            enabled: false,
            text: "My Text".to_string(),
            fontsize: 80,
            color: "white".to_string(),
            position: TextPosition::TopCenter,
        }
    }

    fn xy(&self) -> &str {
        match self.position {
            TextPosition::TopLeft => "x=10:y=10",
            TextPosition::TopCenter => "x=(w-text_w)/2:y=10",
            TextPosition::TopRight => "x=w-text_w:y=10",
            TextPosition::MiddleLeft => "x=10:y=(h-text_h)/2",
            TextPosition::MiddleCenter => "x=(w-text_w)/2:y=(h-text_h)/2",
            TextPosition::MiddleRight => "x=w-text_w-10:y=(h-text_h)/2",
            TextPosition::BottomLeft => "x=10:y=h-text_h-10",
            TextPosition::BottomCenter => "x=(w-text_w)/2:y=h-text_h-10",
            TextPosition::BottomRight => "x=w-text_w-10:y=h-text_h-10",
        }
    }

    pub fn ff_drawtext(&self) -> String {
        format!(
            "drawtext=text='{}':{}:fontsize={}:fontcolor={}",
            self.text,
            self.xy(),
            self.fontsize,
            self.color
        )
    }
}
