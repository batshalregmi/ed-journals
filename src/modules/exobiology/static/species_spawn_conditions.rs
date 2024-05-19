use lazy_static::lazy_static;
use crate::modules::exobiology::models::spawn_condition::SpawnCondition;
use crate::modules::exobiology::Species;
use crate::modules::galaxy::{AtmosphereType, PlanetClass, StarClass, StarLuminosity, VolcanismType};

lazy_static! {
    pub static ref SPECIES_SPAWN_CONDITIONS: [(Species, Vec<SpawnCondition>); 124] = [
        (Species::AleoidaArcus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(175.0),
            SpawnCondition::MaxMeanTemperature(180.0),
        ]),
        (Species::AleoidaCoronamus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(180.0),
            SpawnCondition::MaxMeanTemperature(190.0),
        ]),
        (Species::AleoidaGravis, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(190.0),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::AleoidaLaminiae, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::AleoidaSpica, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::AmphoraPlant, vec![
            SpawnCondition::ParentStarClass(StarClass::A),
            SpawnCondition::NoAtmosphere,
            SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
            SpawnCondition::Any(vec![
                SpawnCondition::SystemContainsPlanetClass(PlanetClass::EarthlikeBody),
                SpawnCondition::SystemContainsPlanetClass(PlanetClass::AmmoniaWorld),
                SpawnCondition::SystemContainsPlanetClass(PlanetClass::GasGiantWithWaterBasedLife),
                SpawnCondition::SystemContainsPlanetClass(PlanetClass::GasGiantWithAmmoniaBasedLife),
                SpawnCondition::SystemContainsPlanetClass(PlanetClass::WaterGiant),
            ]),
        ]),
        (Species::AnemonePrasinus, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::O),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            ]),
        ]),
        (Species::AnemonePrasinumBioluminescent, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::O),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            ]),
        ]),
        (Species::AnemonePuniceus, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::O),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
            ]),
        ]),
        (Species::AnemonePuniceum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::O),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
            ]),
        ]),
        (Species::AnemoneRoseus, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::B),
            SpawnCondition::Any(vec![
                SpawnCondition::ParentStarLuminosity(StarLuminosity::I),
                SpawnCondition::ParentStarLuminosity(StarLuminosity::II),
                SpawnCondition::ParentStarLuminosity(StarLuminosity::III),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            ]),
        ]),
        (Species::AnemoneRoseumBioluminescent, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::B),
            SpawnCondition::Any(vec![
                SpawnCondition::ParentStarLuminosity(StarLuminosity::I),
                SpawnCondition::ParentStarLuminosity(StarLuminosity::II),
                SpawnCondition::ParentStarLuminosity(StarLuminosity::III),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
        ]),
        (Species::AnemoneRoseum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::B),
            SpawnCondition::Any(vec![
                SpawnCondition::ParentStarLuminosity(StarLuminosity::I),
                SpawnCondition::ParentStarLuminosity(StarLuminosity::II),
                SpawnCondition::ParentStarLuminosity(StarLuminosity::III),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
        ]),
        (Species::AnemoneBlattinus, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::B),
            SpawnCondition::Any(vec![
                SpawnCondition::ParentStarLuminosity(StarLuminosity::IV),
                SpawnCondition::ParentStarLuminosity(StarLuminosity::V),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
        ]),
        (Species::AnemoneBlatteumBioluminescent, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::B),
            SpawnCondition::Any(vec![
                SpawnCondition::ParentStarLuminosity(StarLuminosity::IV),
                SpawnCondition::ParentStarLuminosity(StarLuminosity::V),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
        ]),
        (Species::AnemoneLuteus, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::B),
            SpawnCondition::Any(vec![
                SpawnCondition::ParentStarLuminosity(StarLuminosity::IV),
                SpawnCondition::ParentStarLuminosity(StarLuminosity::V),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
        ]),
        (Species::AnemoneLuteolum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::B),
            SpawnCondition::Any(vec![
                SpawnCondition::ParentStarLuminosity(StarLuminosity::IV),
                SpawnCondition::ParentStarLuminosity(StarLuminosity::V),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
        ]),
        (Species::AnemoneRubens, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::Any(vec![
                SpawnCondition::All(vec![
                    SpawnCondition::ParentStarClass(StarClass::B),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::IV),
                ]),
                SpawnCondition::All(vec![
                    SpawnCondition::ParentStarClass(StarClass::A),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::III),
                ]),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
        ]),
        (Species::AnemoneRubeumBioluminescent, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::B),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
        ]),
        (Species::AnemoneCroceus, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::B),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
        ]),
        (Species::AnemoneCroceum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::ParentStarClass(StarClass::B),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
        ]),
        (Species::BarkMound, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::WithinNebulaRange(150.0),
        ]),
        (Species::BacteriumNebulus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Helium),
        ]),
        (Species::BacteriumAcies, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::NeonRich),
            ]),
        ]),
        (Species::BacteriumOmentum, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::NeonRich),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::VolcanismType(VolcanismType::NitrogenMagma),
                SpawnCondition::VolcanismType(VolcanismType::NitrogenGeysers),
                SpawnCondition::VolcanismType(VolcanismType::AmmoniaMagma),
                SpawnCondition::VolcanismType(VolcanismType::AmmoniaGeysers),
            ]),
        ]),
        (Species::BacteriumScopulum, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::NeonRich),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::VolcanismType(VolcanismType::CarbonDioxideGeysers),
                SpawnCondition::VolcanismType(VolcanismType::MethaneGeysers),
                SpawnCondition::VolcanismType(VolcanismType::MethaneMagma),
            ]),
        ]),
        (Species::BacteriumVerrata, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::NeonRich),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::VolcanismType(VolcanismType::WaterMagma),
                SpawnCondition::VolcanismType(VolcanismType::WaterGeysers),
            ]),
        ]),
        (Species::BacteriumBullaris, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Methane),
                SpawnCondition::ThinAtmosphere(AtmosphereType::MethaneRich),
            ]),
        ]),
        (Species::BacteriumAlcyoneum, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
        ]),
        (Species::BacteriumVesicula, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::ArgonRich),
            ]),
        ]),
        (Species::BacteriumCerbrus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
        ]),
        (Species::BacteriumAurasus, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxideRich),
            ]),
        ]),
        (Species::BacteriumInformem, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Nitrogen),
        ]),
        (Species::BacteriumVolu, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Oxygen),
        ]),
        (Species::BacteriumTela, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::VolcanismType(VolcanismType::HeliumGeysers),
                SpawnCondition::VolcanismType(VolcanismType::SilicateMagma),
                SpawnCondition::VolcanismType(VolcanismType::SilicateVapourGeysers),
            ]),
        ]),
        (Species::BrainTreeAureum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::AnyVolcanism,
        ]),
        (Species::BrainTreeOstrinum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::AnyVolcanism,
        ]),
        (Species::BrainTreePuniceum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::AnyVolcanism,
        ]),
        (Species::BrainTreeLindigoticum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::AnyVolcanism,
        ]),
        (Species::BrainTreeGypseeum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::AnyVolcanism,
        ]),
        (Species::BrainTreeLividum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::AnyVolcanism,
        ]),
        (Species::BrainTreeViride, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::AnyVolcanism,
        ]),
        (Species::BrainTreeRoseum, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::AnyVolcanism,
        ]),
        (Species::CactoidaLapis, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
        ]),
        (Species::CactoidaPullulanta, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxideRich),
            ]),
            SpawnCondition::MinMeanTemperature(180.0),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::CactoidaCortexum, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxideRich),
            ]),
            SpawnCondition::MinMeanTemperature(180.0),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::CactoidaVermis, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
        ]),
        (Species::CactoidaPeperatis, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
        ]),
        (Species::ClypeusSpeculumi, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(190.0),
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
            ]),
            SpawnCondition::MinDistanceFromParentSun(5.0),
        ]),
        (Species::ClypeusLacrimam, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(190.0),
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
            ]),
        ]),
        (Species::ClypeusMargaritus, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(190.0),
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
            ]),
        ]),
        (Species::ConchaRenibus, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::All(vec![
                    SpawnCondition::MaxGravity(0.27),
                    SpawnCondition::Any(vec![
                        SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                        SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxideRich),
                    ]),
                    SpawnCondition::MinMeanTemperature(180.0),
                    SpawnCondition::MaxMeanTemperature(195.0),
                ]),
                SpawnCondition::All(vec![
                    SpawnCondition::MaxGravity(0.27),
                    SpawnCondition::Any(vec![
                        SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                        SpawnCondition::ThinAtmosphere(AtmosphereType::WaterRich),
                    ]),
                ]),
            ]),
        ]),
        (Species::ConchaAureolas, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
        ]),
        (Species::ConchaLabiata, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxideRich),
            ]),
            SpawnCondition::MaxMeanTemperature(190.0),
        ]),
        (Species::ConchaBiconcavis, vec![
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::ThinAtmosphere(AtmosphereType::Nitrogen),
        ]),
        (Species::CrystallineShards, vec![
            SpawnCondition::NoAtmosphere,
            SpawnCondition::Any(vec![
                SpawnCondition::ParentStarClass(StarClass::A),
                SpawnCondition::ParentStarClass(StarClass::F),
                SpawnCondition::ParentStarClass(StarClass::G),
                SpawnCondition::ParentStarClass(StarClass::K),
                SpawnCondition::ParentStarClass(StarClass::M),
                SpawnCondition::ParentStarClass(StarClass::S),
            ]),
            SpawnCondition::MinMeanTemperature(0.0),
            SpawnCondition::MaxMeanTemperature(273.0),
            SpawnCondition::Any(vec![
                SpawnCondition::SystemContainsPlanetClass(PlanetClass::EarthlikeBody),
                SpawnCondition::SystemContainsPlanetClass(PlanetClass::AmmoniaWorld),
                SpawnCondition::SystemContainsPlanetClass(PlanetClass::GasGiantWithWaterBasedLife),
                SpawnCondition::SystemContainsPlanetClass(PlanetClass::GasGiantWithAmmoniaBasedLife),
                SpawnCondition::SystemContainsPlanetClass(PlanetClass::WaterGiant),
            ]),
            SpawnCondition::MinDistanceFromParentSun(24.0),
        ]),
        (Species::ElectricaePluma, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Helium),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::IcyBody),
            SpawnCondition::ParentStarClass(StarClass::A),
            SpawnCondition::Any(vec![
                SpawnCondition::MinOrEqualParentStarLuminosity(StarLuminosity::V),
                SpawnCondition::ParentStarClass(StarClass::N),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::ElectricaeRadialem, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Helium),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::IcyBody),
            SpawnCondition::WithinNebulaRange(150.0),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FonticuluaCampestris, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FonticuluaSegmentatus, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::NeonRich),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FonticuluaDigitos, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Methane),
                SpawnCondition::ThinAtmosphere(AtmosphereType::MethaneRich),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FonticuluaUpupam, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FonticuluaLapida, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Nitrogen),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FonticuluaFluctus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Oxygen),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FrutexaAcus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::FrutexaCollum, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FrutexaFera, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::FrutexaFlabellum, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FrutexaFlammasis, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FrutexaMetallicum, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::All(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                    SpawnCondition::MaxMeanTemperature(195.0),
                ]),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FrutexaSponsae, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FumerolaAquatis, vec![
            SpawnCondition::AnyThinAtmosphere,
            SpawnCondition::GeologicalSignalsPresent,
            SpawnCondition::Any(vec![
                SpawnCondition::VolcanismType(VolcanismType::WaterMagma),
                SpawnCondition::VolcanismType(VolcanismType::WaterGeysers),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FumerolaCarbosis, vec![
            SpawnCondition::AnyThinAtmosphere,
            SpawnCondition::GeologicalSignalsPresent,
            SpawnCondition::Any(vec![
                SpawnCondition::VolcanismType(VolcanismType::CarbonDioxideGeysers),
                SpawnCondition::VolcanismType(VolcanismType::MethaneMagma),
                SpawnCondition::VolcanismType(VolcanismType::MethaneGeysers),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FumerolaExtremus, vec![
            SpawnCondition::AnyThinAtmosphere,
            SpawnCondition::GeologicalSignalsPresent,
            SpawnCondition::Any(vec![
                SpawnCondition::VolcanismType(VolcanismType::CarbonDioxideGeysers),
                SpawnCondition::VolcanismType(VolcanismType::MethaneMagma),
                SpawnCondition::VolcanismType(VolcanismType::MethaneGeysers),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FumerolaNitris, vec![
            SpawnCondition::AnyThinAtmosphere,
            SpawnCondition::GeologicalSignalsPresent,
            SpawnCondition::Any(vec![
                SpawnCondition::VolcanismType(VolcanismType::NitrogenMagma),
                SpawnCondition::VolcanismType(VolcanismType::NitrogenGeysers),
                SpawnCondition::VolcanismType(VolcanismType::AmmoniaMagma),
                SpawnCondition::VolcanismType(VolcanismType::AmmoniaGeysers),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FungoidaBullarum, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::ArgonRich),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FungoidaGelata, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                SpawnCondition::All(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::ArgonRich),
                    SpawnCondition::MinMeanTemperature(180.0),
                    SpawnCondition::MaxMeanTemperature(195.0),
                ]),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FungoidaSetisis, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Methane),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::FungoidaStabitis, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                SpawnCondition::All(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::ArgonRich),
                    SpawnCondition::MinMeanTemperature(180.0),
                    SpawnCondition::MaxMeanTemperature(195.0),
                ]),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::OsseusCornibus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(180.0),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::OsseusDiscus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::OsseusFractus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(180.0),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::OsseusPellebantus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(180.0),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::OsseusPumice, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Methane),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Nitrogen),
            ]),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::OsseusSpiralis, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::ReceptaConditivus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::ReceptaDeltahedronix, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::ReceptaUmbrux, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::SinuousTubersAlbidum, vec![
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
        ]),
        (Species::SinuousTubersBlatteum, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
        ]),
        (Species::SinuousTubersCaeruleum, vec![
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
        ]),
        (Species::SinuousTubersLindigoticum, vec![
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
        ]),
        (Species::SinuousTubersPrasinum, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
        ]),
        (Species::SinuousTubersRoseum, vec![
            SpawnCondition::VolcanismType(VolcanismType::SilicateMagma),
        ]),
        (Species::SinuousTubersViolaceum, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
        ]),
        (Species::SinuousTubersViride, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            ]),
        ]),
        (Species::StratumAraneamus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MinMeanTemperature(165.0),
        ]),
        (Species::StratumCucumisis, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MinMeanTemperature(190.0),
        ]),
        (Species::StratumExcutitus, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MinMeanTemperature(165.0),
            SpawnCondition::MaxMeanTemperature(190.0),
        ]),
        (Species::StratumFrigus, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MinMeanTemperature(190.0),
        ]),
        (Species::StratumLaminamus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MinMeanTemperature(165.0),
        ]),
        (Species::StratumLimaxus, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MinMeanTemperature(165.0),
            SpawnCondition::MaxMeanTemperature(190.0),
        ]),
        (Species::StratumPaleas, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MinMeanTemperature(165.0),
        ]),
        (Species::StratumTectonicas, vec![
            SpawnCondition::AnyThinAtmosphere,
            SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            SpawnCondition::MinMeanTemperature(165.0),
        ]),
        (Species::TubusCavas, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.15),
            SpawnCondition::MinMeanTemperature(160.0),
            SpawnCondition::MaxMeanTemperature(190.0),
        ]),
        (Species::TubusCompagibus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.15),
            SpawnCondition::MinMeanTemperature(160.0),
            SpawnCondition::MaxMeanTemperature(190.0),
        ]),
        (Species::TubusConifer, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.15),
            SpawnCondition::MinMeanTemperature(160.0),
            SpawnCondition::MaxMeanTemperature(190.0),
        ]),
        (Species::TubusRosarium, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.15),
            SpawnCondition::MinMeanTemperature(160.0),
        ]),
        (Species::TubusSororibus, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
            SpawnCondition::MaxGravity(0.15),
            SpawnCondition::MinMeanTemperature(160.0),
            SpawnCondition::MaxMeanTemperature(190.0),
        ]),
        (Species::TussockAlbata, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(175.0),
            SpawnCondition::MaxMeanTemperature(180.0),
        ]),
        (Species::TussockCapillum, vec![
            SpawnCondition::Any(vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Methane),
            ]),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::TussockCaputus, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(180.0),
            SpawnCondition::MaxMeanTemperature(190.0),
        ]),
        (Species::TussockCatena, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::TussockCultro, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::TussockDivisa, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::TussockIgnis, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(160.0),
            SpawnCondition::MaxMeanTemperature(170.0),
        ]),
        (Species::TussockPennata, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(145.0),
            SpawnCondition::MaxMeanTemperature(155.0),
        ]),
        (Species::TussockPennatis, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::TussockPropagito, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::TussockSerrati, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(170.0),
            SpawnCondition::MaxMeanTemperature(175.0),
        ]),
        (Species::TussockStigmasis, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
        (Species::TussockTriticum, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(190.0),
            SpawnCondition::MaxMeanTemperature(195.0),
        ]),
        (Species::TussockVentusa, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
            SpawnCondition::MinMeanTemperature(155.0),
            SpawnCondition::MaxMeanTemperature(160.0),
        ]),
        (Species::TussockVirgam, vec![
            SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
            SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            SpawnCondition::MaxGravity(0.27),
        ]),
    ];
}
