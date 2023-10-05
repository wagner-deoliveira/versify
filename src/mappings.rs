use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum App {
    SATK,
    Mashup,
    SSC,
    SSIV,
    SCE,
    HCS,
    ImageImport,
    ImageDiscovery,
    SciStream,
    Metastore,
}

impl FromStr for App {
    type Err = ();

    fn from_str(app: &str) -> Result<Self, Self::Err> {
        match app {
            "SATK" => Ok(App::SATK),
            "Mashup" => Ok(App::Mashup),
            "SSC" => Ok(App::SSC),
            "SSIV" => Ok(App::SSIV),
            "SCE" => Ok(App::SCE),
            "HCS" => Ok(App::HCS),
            "ImageImport" => Ok(App::ImageImport),
            "ImageDiscovery" => Ok(App::ImageDiscovery),
            "SciStream" => Ok(App::SciStream),
            "Metastore" => Ok(App::Metastore),
            _ => Err(())
        }
    }
}

pub fn inspect_app(app: &App) -> Vec<&'static str> {
    match app {
        App::SATK => {
            vec![
                "PerkinElmer Signals Analytics Apps",
                "PerkinElmer Signals Groups",
                "PerkinElmer Signals Platform Cobranding",
                "Signals SaaS",
            ]
        }
        App::SSC => {
            vec![
                "PerkinElmer Signals Analytics Common",
                "PerkinElmer Signals Analytics Apps for Screening",
                "PerkinElmer SPR",
                "Signals SPR Zeroing App",
                "PerkinElmer Signals VitroVivo Product",
            ]
        }
        App::SSIV => {
            vec!["PerkinElmer.Signals.InVivo"]
        }
        App::SCE => {
            vec!["PerkinElmer Calculation Explorer"]
        }
        App::HCS => {
            vec!["Integromics.Hcs"]
        }
        App::ImageImport => {
            vec!["Image Import"]
        }
        App::ImageDiscovery => {
            vec!["Image Discovery"]
        }
        App::SciStream => {
            vec!["SciStream Automation"]
        }
        App::Metastore => {
            vec!["PerkinElmer.Signals.Analytics.Utils.Metastore"]
        }
        App::Mashup => {
            vec!["Signals.Spotfire.Mashups.Tool"]
        }
    }
}