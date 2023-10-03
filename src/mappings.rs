use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum App {
    SDK,
    SSC,
    SPR,
    SSIV,
    SCE,
    Integromics,
    Image,
}

impl FromStr for App {
    type Err = ();

    fn from_str(app: &str) -> Result<Self, Self::Err> {
        match app {
            "SDK" => Ok(App::SDK),
            "SSC" => Ok(App::SSC),
            "SPR" => Ok(App::SPR),
            "SSIV" => Ok(App::SSIV),
            "SCE" => Ok(App::SCE),
            "Integromics" => Ok(App::Integromics),
            "Image" => Ok(App::Image),
            _ => Err(())
        }
    }
}

pub fn inspect_app(app: App) -> Vec<&'static str> {
    match app {
        App::SDK => {
            vec![
                "PerkinElmer Signals Analytics Apps:main/latest",
                "PerkinElmer Signals Groups:main/latest",
                "PerkinElmer Signals Platform Cobranding:main/latest",
                "Signals SaaS:main/latest",
                "Signals.Spotfire.Mashups.Tool:main/latest",
            ]
        }
        App::SSC => {
            vec![
                "PerkinElmer Signals Analytics Common:main/latest",
                "PerkinElmer Signals Analytics Apps for Screening:main/latest",
            ]
        }
        App::SPR => {
            vec![
                "PerkinElmer SPR Alignment App:main/latest",
                "PerkinElmer SPR Blank Substraction App:main/latest",
                "PerkinElmer SPR Common:main/latest",
                "PerkinElmer SPR Cropping App:main/latest",
                "PerkinElmer SPR Data Readers:main/latest",
                "PerkinElmer SPR Export Report App:main/latest",
                "PerkinElmer SPR Hit Selection App:main/latest",
                "PerkinElmer SPR Reference App:main/latest",
                "PerkinElmer SPR RAC App:main/latest",
                "PerkinElmer SPR Relative Potency App:main/latest",
                "PerkinElmer SPR Single Cycle Kinetics App:main/latest",
                "PerkinElmer SPR MultiCycle Kinetic Analysis App:main/latest",
                "PerkinElmer SPR ReportPoints App:main/latest",
                "PerkinElmer SPR Solvent Correction App:main/latest",
                "PerkinElmer SPR TraceDrawer Export App:main/latest",
                "PerkinElmer SPR Steady State Analysis App:main/latest",
                "PerkinElmer SPR Zeroing App:main/latest",
                "Signals SPR Zeroing App:main/latest",
                "PerkinElmer SPR Data Import SDK App:main/latest",
            ]
        }
        App::SSIV => {
            vec![
                "PerkinElmer.Signals.InVivo.BaselineCapture:main/latest",
                "PerkinElmer.Signals.InVivo.BusinessRules:main/latest",
                "PerkinElmer.Signals.InVivo.Common:main/latest",
                "PerkinElmer.Signals.InVivo.DataModelDesigner:main/latest",
                "PerkinElmer.Signals.InVivo.DosePreparation:main/latest",
                "PerkinElmer.Signals.InVivo.DoseVerification:main/latest",
                "PerkinElmer.Signals.InVivo.MassSpec:main/latest",
                "PerkinElmer.Signals.InVivo.PKParameters:main/latest",
                "PerkinElmer.Signals.InVivo.SequenceFileGenerator:main/latest",
                "PerkinElmer.Signals.InVivo.SequenceOfEvents:main/latest",
                "PerkinElmer.Signals.InVivo.StudyDesigner:main/latest",
            ]
        }
        App::SCE => {
            vec!["PerkinElmer Calculation Explorer:main/latest"]
        }
        App::Integromics => {
            vec![
                "Integromics.Hcs.Help:main/latest",
                "Integromics.Hcs.Python:main/latest",
                "Integromics.Hcs.Web:main/latest",
                "Integromics.Hcs:main/latest",
            ]
        }
        App::Image => {
            vec![
                "Image Import:main/latest",
                "Image Discovery:main/latest",
            ]
        }
    }
}