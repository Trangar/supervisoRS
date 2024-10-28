#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub enum Flags {
    Hidden,
    OnlyInCursor,
    PrimaryPlaceResult,
    AlwaysShow,
    DrawLogisticOverlay,
    HideFromBonusGui,
    PlayerCreation,
    PlaceableNeutral,
    PlaceablePlayer,
    NotRotatable,
    HideAltInfo,
    GoesToMainInventory,
    NotStackable,
    Spawnable,
    NotRepairable,
    NotDeconstructable,
}

impl std::str::FromStr for Flags {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hidden" | "Hidden" => Ok(Flags::Hidden),
            "only-in-cursor" | "OnlyInCursor" => Ok(Flags::OnlyInCursor),
            "primary-place-result" | "PrimaryPlaceResult" => Ok(Flags::PrimaryPlaceResult),
            "always-show" | "AlwaysShow" => Ok(Flags::AlwaysShow),
            "draw-logistic-overlay" | "DrawLogisticOverlay" => Ok(Flags::DrawLogisticOverlay),
            "hide-from-bonus-gui" | "HideFromBonusGui" => Ok(Flags::HideFromBonusGui),
            "player-creation" | "PlayerCreation" => Ok(Flags::PlayerCreation),
            "placeable-neutral" | "PlaceableNeutral" => Ok(Flags::PlaceableNeutral),
            "placeable-player" | "PlaceablePlayer" => Ok(Flags::PlaceablePlayer),
            "not-rotatable" | "NotRotatable" => Ok(Flags::NotRotatable),
            "hide-alt-info" | "HideAltInfo" => Ok(Flags::HideAltInfo),
            "goes-to-main-inventory" | "GoesToMainInventory" => Ok(Flags::GoesToMainInventory),
            "not-stackable" | "NotStackable" => Ok(Flags::NotStackable),
            "spawnable" | "Spawnable" => Ok(Flags::Spawnable),
            "not-repairable" | "NotRepairable" => Ok(Flags::NotRepairable),
            "not-deconstructable" | "NotDeconstructable" => Ok(Flags::NotDeconstructable),
            _ => Err(()),
        }
    }
}

impl Flags {
    pub fn all() -> &'static [Self] {
        &[
            Flags::Hidden,
            Flags::OnlyInCursor,
            Flags::PrimaryPlaceResult,
            Flags::AlwaysShow,
            Flags::DrawLogisticOverlay,
            Flags::HideFromBonusGui,
            Flags::PlayerCreation,
            Flags::PlaceableNeutral,
            Flags::PlaceablePlayer,
            Flags::NotRotatable,
            Flags::HideAltInfo,
            Flags::GoesToMainInventory,
            Flags::NotStackable,
            Flags::Spawnable,
            Flags::NotRepairable,
            Flags::NotDeconstructable,
        ]
    }

    fn all_names() -> &'static [&'static str] {
        &[
            "hidden",
            "only-in-cursor",
            "primary-place-result",
            "always-show",
            "draw-logistic-overlay",
            "hide-from-bonus-gui",
            "player-creation",
            "placeable-neutral",
            "placeable-player",
            "not-rotatable",
            "hide-alt-info",
            "goes-to-main-inventory",
            "not-stackable",
            "spawnable",
            "not-repairable",
            "not-deconstructable",
        ]
    }
}

impl<'de> serde::de::Deserialize<'de> for Flags {
    fn deserialize<D>(deserializer: D) -> Result<Flags, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Flags;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string flag")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse()
                    .map_err(|_| serde::de::Error::unknown_variant(v, Flags::all_names()))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
