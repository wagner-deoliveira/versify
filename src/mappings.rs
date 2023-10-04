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

pub fn inspect_app(app: App) -> Vec<&'static str> {
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
                "PerkinElmer SPR Alignment App",
                "PerkinElmer SPR Blank Substraction App",
                "PerkinElmer SPR Common",
                "PerkinElmer SPR Cropping App",
                "PerkinElmer SPR Data Readers",
                "PerkinElmer SPR Export Report App",
                "PerkinElmer SPR Hit Selection App",
                "PerkinElmer SPR Reference App",
                "PerkinElmer SPR RAC App",
                "PerkinElmer SPR Relative Potency App",
                "PerkinElmer SPR Single Cycle Kinetics App",
                "PerkinElmer SPR MultiCycle Kinetic Analysis App",
                "PerkinElmer SPR ReportPoints App",
                "PerkinElmer SPR Solvent Correction App",
                "PerkinElmer SPR TraceDrawer Export App",
                "PerkinElmer SPR Steady State Analysis App",
                "PerkinElmer SPR Zeroing App",
                "Signals SPR Zeroing App",
                "PerkinElmer SPR Data Import SDK App",
                "PerkinElmer Signals VitroVivo Product",
            ]
        }
        App::SSIV => {
            vec![
                "PerkinElmer.Signals.InVivo.BaselineCapture",
                "PerkinElmer.Signals.InVivo.BusinessRules",
                "PerkinElmer.Signals.InVivo.Common",
                "PerkinElmer.Signals.InVivo.DataModelDesigner",
                "PerkinElmer.Signals.InVivo.DosePreparation",
                "PerkinElmer.Signals.InVivo.DoseVerification",
                "PerkinElmer.Signals.InVivo.MassSpec",
                "PerkinElmer.Signals.InVivo.PKParameters",
                "PerkinElmer.Signals.InVivo.SequenceFileGenerator",
                "PerkinElmer.Signals.InVivo.SequenceOfEvents",
                "PerkinElmer.Signals.InVivo.StudyDesigner",
            ]
        }
        App::SCE => {
            vec!["PerkinElmer Calculation Explorer"]
        }
        App::HCS => {
            vec![
                "Integromics.Hcs.Help",
                "Integromics.Hcs.Python",
                "Integromics.Hcs.Web",
                "Integromics.Hcs",
            ]
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