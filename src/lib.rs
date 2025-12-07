use gloo::console::log;
use gloo::storage::{LocalStorage, Storage};
use gloo::utils::document;
use js_sys::Array;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, window, Request, RequestInit, RequestMode, Response};
use yew::prelude::*;

// --- Structures de données ---
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Category {
    id: String,
    name: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Subcategory {
    id: String,
    name: String,
    parent_id: String,
    #[serde(rename = "type")]
    sub_type: String, // Renommé pour éviter conflit avec mot-clé Rust
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Command {
    id: String,
    subcat: String,
    action: String,
    description: String,
    syntaxes: Option<Vec<String>>,
    examples: Option<Vec<String>>,
    tips: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct InitialData {
    categories: Vec<Category>,
    subcategories: Vec<Subcategory>,
    commands: Vec<Command>,
}

// --- Données initiales (votre INITIAL_DATA) ---
const INITIAL_DATA_JSON: &str = r#"
{
    "categories": [
        { "id": "syntax", "name": "Syntaxe & Symboles", "icon": "🔤" },
        { "id": "live-blind", "name": "Live / Blind", "icon": "👁️" },
        { "id": "patch", "name": "Patch & Fixtures", "icon": "📡" },
        { "id": "groups", "name": "Groups", "icon": "👥" },
        { "id": "palettes", "name": "Palettes (IFCB)", "icon": "🎨" },
        { "id": "submasters", "name": "Submasters", "icon": "🔌" },
        { "id": "cues", "name": "Cues & Cue Lists", "icon": "🎬" },
        { "id": "effects", "name": "Effects", "icon": "🌀" },
        { "id": "macros", "name": "Macros", "icon": "⚙️" },
        { "id": "magic-sheets", "name": "Magic Sheets", "icon": "📝" },
        { "id": "show-control", "name": "Show Control", "icon": "📡" },
        { "id": "diagnostics", "name": "Diagnostics", "icon": "🔍" },
        { "id": "advanced", "name": "Contrôle Avancé", "icon": "⚡" },
        { "id": "emergency", "name": "Commandes d'Urgence", "icon": "🚨" },
        { "id": "pixels-media", "name": "Pixels & Media", "icon": "🌈" },
        { "id": "timecode", "name": "Time Code", "icon": "⏱️" },
        { "id": "network", "name": "Réseau", "icon": "🌐" },
        { "id": "events", "name": "Événements", "icon": "📅" },
        { "id": "editing", "name": "Édition", "icon": "✏️" },
        { "id": "partition", "name": "Partitions", "icon": "🔒" },
        { "id": "file", "name": "Fichiers", "icon": "📁" }
    ],
    "subcategories": [
        { "id": "syntax-base", "name": "Base", "parentId": "syntax", "type": "base" },
        { "id": "syntax-user", "name": "Utilisateur", "parentId": "syntax", "type": "user" },
        { "id": "live-blind-base", "name": "Base", "parentId": "live-blind", "type": "base" },
        { "id": "live-blind-user", "name": "Utilisateur", "parentId": "live-blind", "type": "user" },
        { "id": "patch-base", "name": "Base", "parentId": "patch", "type": "base" },
        { "id": "patch-user", "name": "Utilisateur", "parentId": "patch", "type": "user" },
        { "id": "groups-base", "name": "Base", "parentId": "groups", "type": "base" },
        { "id": "groups-user", "name": "Utilisateur", "parentId": "groups", "type": "user" },
        { "id": "palettes-base", "name": "Base", "parentId": "palettes", "type": "base" },
        { "id": "palettes-user", "name": "Utilisateur", "parentId": "palettes", "type": "user" },
        { "id": "submasters-base", "name": "Base", "parentId": "submasters", "type": "base" },
        { "id": "submasters-user", "name": "Utilisateur", "parentId": "submasters", "type": "user" },
        { "id": "cues-base", "name": "Base", "parentId": "cues", "type": "base" },
        { "id": "cues-user", "name": "Utilisateur", "parentId": "cues", "type": "user" },
        { "id": "effects-base", "name": "Base", "parentId": "effects", "type": "base" },
        { "id": "effects-user", "name": "Utilisateur", "parentId": "effects", "type": "user" },
        { "id": "macros-base", "name": "Base", "parentId": "macros", "type": "base" },
        { "id": "macros-user", "name": "Utilisateur", "parentId": "macros", "type": "user" },
        { "id": "magic-sheets-base", "name": "Base", "parentId": "magic-sheets", "type": "base" },
        { "id": "magic-sheets-user", "name": "Utilisateur", "parentId": "magic-sheets", "type": "user" },
        { "id": "show-control-base", "name": "Base", "parentId": "show-control", "type": "base" },
        { "id": "show-control-user", "name": "Utilisateur", "parentId": "show-control", "type": "user" },
        { "id": "diagnostics-base", "name": "Base", "parentId": "diagnostics", "type": "base" },
        { "id": "diagnostics-user", "name": "Utilisateur", "parentId": "diagnostics", "type": "user" },
        { "id": "advanced-base", "name": "Base", "parentId": "advanced", "type": "base" },
        { "id": "advanced-user", "name": "Utilisateur", "parentId": "advanced", "type": "user" },
        { "id": "emergency-base", "name": "Base", "parentId": "emergency", "type": "base" },
        { "id": "emergency-user", "name": "Utilisateur", "parentId": "emergency", "type": "user" },
        { "id": "pixels-media-base", "name": "Base", "parentId": "pixels-media", "type": "base" },
        { "id": "pixels-media-user", "name": "Utilisateur", "parentId": "pixels-media", "type": "user" },
        { "id": "timecode-base", "name": "Base", "parentId": "timecode", "type": "base" },
        { "id": "timecode-user", "name": "Utilisateur", "parentId": "timecode", "type": "user" },
        { "id": "network-base", "name": "Base", "parentId": "network", "type": "base" },
        { "id": "network-user", "name": "Utilisateur", "parentId": "network", "type": "user" },
        { "id": "events-base", "name": "Base", "parentId": "events", "type": "base" },
        { "id": "events-user", "name": "Utilisateur", "parentId": "events", "type": "user" },
        { "id": "editing-base", "name": "Base", "parentId": "editing", "type": "base" },
        { "id": "editing-user", "name": "Utilisateur", "parentId": "editing", "type": "user" },
        { "id": "partition-base", "name": "Base", "parentId": "partition", "type": "base" },
        { "id": "partition-user", "name": "Utilisateur", "parentId": "partition", "type": "user" },
        { "id": "file-base", "name": "Base", "parentId": "file", "type": "base" },
        { "id": "file-user", "name": "Utilisateur", "parentId": "file", "type": "user" }
    ],
    "commands": [
        {
            "id": "at",
            "subcat": "syntax-base",
            "action": "Touche [At]",
            "description": "Définit un niveau (intensité, position, etc.).",
            "syntaxes": [
                "[1][At][50][Enter]"
            ],
            "examples": [
                "[1][At][50][Enter] → met le canal 1 à 50%."
            ],
            "tips": [
                "Toujours nécessite [Enter] sauf pour [Full][Full]."
            ]
        },
        {
            "id": "thru",
            "subcat": "syntax-base",
            "action": "Touche [Thru]",
            "description": "Sélectionne une plage de canaux.",
            "syntaxes": [
                "[1][Thru][10]"
            ],
            "examples": [
                "[1][Thru][10][At][Full][Enter] → allume les canaux 1 à 10 à 100%."
            ],
            "tips": [
                "Très utile pour grouper des commandes."
            ]
        },
        {
            "id": "full",
            "subcat": "syntax-base",
            "action": "Touche [Full]",
            "description": "Raccourci pour [At][Full][Enter].",
            "syntaxes": [
                "[1][Full][Full]",
                "[1][Thru][10][Full][Full]"
            ],
            "examples": [
                "[1][Full][Full] → met le canal 1 à 100%.",
                "[1][Thru][10][Full][Full] → met les canaux 1 à 10 à 100%."
            ],
            "tips": [
                "[Full][Full] = [At][Full][Enter] en raccourci."
            ]
        },
        {
            "id": "out",
            "subcat": "syntax-base",
            "action": "Touche [Out]",
            "description": "Raccourci pour [At][0][Enter].",
            "syntaxes": [
                "[1][Out][Out]",
                "[1][Thru][10][Out][Out]"
            ],
            "examples": [
                "[1][Out][Out] → met le canal 1 à 0%.",
                "[1][Thru][10][Out][Out] → met les canaux 1 à 10 à 0%."
            ],
            "tips": [
                "[Out][Out] = [At][0][Enter] en raccourci."
            ]
        },
        {
            "id": "softkey",
            "subcat": "syntax-base",
            "action": "Softkey { }",
            "description": "Bouton à l’écran, accessible via les touches sous l’écran.",
            "syntaxes": [
                "{Hold}",
                "{Solo}",
                "{Make Absolute}"
            ],
            "examples": [
                "{Hold} après enregistrement d'un submaster."
            ],
            "tips": [
                "Les softkeys changent selon le contexte."
            ]
        },
        {
            "id": "user-0",
            "subcat": "syntax-base",
            "action": "Utilisateur en arrière-plan <U0>",
            "description": "Exécute une commande sans apparaître sur la ligne principale.",
            "syntaxes": [
                "<U0> Clear Sneak 1 Enter"
            ],
            "examples": [
                "Bouton 'Clean Sneak' sur Magic Sheet."
            ],
            "tips": [
                "Syntaxe : `<U0>$ [commande]` (espace après `$`)."
            ]
        },
        {
            "id": "direct-select",
            "subcat": "syntax-base",
            "action": "Direct Select « »",
            "description": "Boutons physiques en haut de la console.",
            "syntaxes": [
                "« 1 »",
                "« House »"
            ],
            "examples": [
                "Stocker la palette 'Open' sur le Direct Select 1."
            ],
            "tips": [
                "Configurer via Setup > Direct Selects."
            ]
        },
        {
            "id": "enter",
            "subcat": "syntax-base",
            "action": "Touche [Enter]",
            "description": "Valide la commande. Certaines commandes s’auto-valident.",
            "syntaxes": [
                "[Full][Full]",
                "[1][Thru][10][At][Full][Enter]"
            ],
            "examples": [
                "[1][Thru][10][At][Full][Enter] → allume les canaux 1–10 à 100%.",
                "[Full][Full] = [At][Full][Enter] en raccourci."
            ],
            "tips": [
                "Toujours nécessite [Enter] sauf pour [Full][Full], [Out][Out], etc."
            ]
        },
        {
            "id": "format",
            "subcat": "syntax-base",
            "action": "Basculer le format d'affichage [Format]",
            "description": "Passe d’un affichage détaillé à un affichage simplifié.",
            "syntaxes": [
                "[Format]"
            ],
            "examples": [
                "[Format] en Live pour voir uniquement l’intensité."
            ],
            "tips": [
                "Utile pour les régisseurs d’accueil."
            ]
        },
        {
            "id": "help",
            "subcat": "syntax-base",
            "action": "Aide contextuelle [Help]",
            "description": "Affiche la fonction, description, et syntaxe d’une touche.",
            "syntaxes": [
                "[Help] + [touche]"
            ],
            "examples": [
                "[Help] + [Sub] affiche l’aide pour les Submasters."
            ],
            "tips": [
                "Fonctionne sur les softkeys et objets Magic Sheet."
            ]
        },
        {
            "id": "clear",
            "subcat": "syntax-base",
            "action": "Effacer la ligne de commande [Clear]",
            "description": "Annule la commande en cours.",
            "syntaxes": [
                "[Clear]"
            ],
            "examples": [
                "[1][Thru][10][Clear] → annule la sélection."
            ],
            "tips": [
                "Ne supprime pas les données stockées."
            ]
        },
        {
            "id": "sneak",
            "subcat": "advanced-base",
            "action": "Touche [Sneak]",
            "description": "Applique un fondu à la valeur manuelle (non enregistrée).",
            "syntaxes": [
                "[1][At][50][Sneak][2][Enter]"
            ],
            "examples": [
                "[1][At][50][Sneak][2][Enter] → le canal 1 monte à 50% en 2 secondes."
            ],
            "tips": [
                "Utile pour des ajustements visuels en douceur."
            ]
        },
        {
            "id": "shift-sneak",
            "subcat": "advanced-base",
            "action": "Touche [Shift] + [Sneak]",
            "description": "Rend les données manuelles non manuelles (désactive Update/Record Only).",
            "syntaxes": [
                "[Shift][Sneak]",
                "[1][Thru][10][Shift][Sneak]"
            ],
            "examples": [
                "[Shift][Sneak] → affecte toutes les données manuelles.",
                "[1][Thru][10][Shift][Sneak] → affecte uniquement les canaux 1-10."
            ],
            "tips": [
                "Les valeurs restent, mais ne sont plus modifiables par Update/Record Only."
            ]
        },
        {
            "id": "staging-mode",
            "subcat": "live-blind-base",
            "action": "Mode Staging",
            "description": "Mode de prévisualisation pour les techniciens.",
            "syntaxes": [
                "Setup > Staging Mode"
            ],
            "examples": [
                "Permet de voir les changements avant la mise en Live."
            ],
            "tips": [
                "Très utile en répétition."
            ]
        },
        {
            "id": "live-mode",
            "subcat": "live-blind-base",
            "action": "Basculer en mode Live",
            "description": "Active la sortie DMX.",
            "syntaxes": [
                "[Live]"
            ],
            "examples": [
                "[Live] → fond doré, sortie active."
            ],
            "tips": [
                "Fond **doré** en Live."
            ]
        },
        {
            "id": "blind-mode",
            "subcat": "live-blind-base",
            "action": "Basculer en mode Blind",
            "description": "Édition sans sortie DMX.",
            "syntaxes": [
                "[Blind]"
            ],
            "examples": [
                "[Blind] → fond bleu, édition sécurisée."
            ],
            "tips": [
                "Fond **bleu** en Blind."
            ]
        },
        {
            "id": "select-last",
            "subcat": "live-blind-base",
            "action": "Sélectionner la dernière sélection",
            "description": "Rappelle la dernière sélection de canaux.",
            "syntaxes": [
                "[Last]"
            ],
            "examples": [
                "[1][Thru][10][At][50][Enter] → [Last] → [At][Full][Enter] → met 1-10 à 100%."
            ],
            "tips": [
                "Très utile pour répéter des actions."
            ]
        },
        {
            "id": "channel-check",
            "subcat": "live-blind-base",
            "action": "Vérifier un canal (Channel Check)",
            "description": "Test visuel temporaire d’un canal.",
            "syntaxes": [
                "[1][At][7]<0>{Chan Check}[Enter]"
            ],
            "examples": [
                "[1][At][7]<0>{Chan Check}[Enter] → amène à 70 %."
            ],
            "tips": [
                "Les valeurs reviennent à leur état d’origine."
            ]
        },
        {
            "id": "address-at-level",
            "subcat": "live-blind-base",
            "action": "Address At Level",
            "description": "Permet de piloter une adresse DMX directement.",
            "syntaxes": [
                "[Address][513][At][50][Enter]"
            ],
            "examples": [
                "[Address][513][At][50][Enter] → met l’adresse 513 à 50 %."
            ],
            "tips": [
                "Utile pour le diagnostic."
            ]
        },
        {
            "id": "address-check",
            "subcat": "live-blind-base",
            "action": "Address Check",
            "description": "Vérifie la valeur d’une adresse DMX.",
            "syntaxes": [
                "[Address][513][At][Check][Enter]"
            ],
            "examples": [
                "[Address][513][At][Check][Enter] → affiche la valeur actuelle."
            ],
            "tips": [
                "Ne modifie pas la valeur."
            ]
        },
        {
            "id": "flash",
            "subcat": "live-blind-base",
            "action": "Flash temporaire",
            "description": "Active un canal tant que la touche est enfoncée.",
            "syntaxes": [
                "[1][Flash]"
            ],
            "examples": [
                "[1][Flash] → identifie le projecteur 1."
            ],
            "tips": [
                "Ne laisse aucune trace.",
                "Fonctionne uniquement en [Live]."
            ]
        },
        {
            "id": "patch-create",
            "subcat": "patch-base",
            "action": "Créer un patch",
            "description": "Associe un canal Eos à une adresse DMX physique.",
            "syntaxes": [
                "[1][Patch][Enter]",
                "[1][Patch][513][Enter]"
            ],
            "examples": [
                "[1][Patch][513][Enter] → canal 1 = adresse DMX 513."
            ],
            "tips": [
                "Essentiel pour contrôler les projecteurs."
            ]
        },
        {
            "id": "patch-edit",
            "subcat": "patch-base",
            "action": "Éditer un patch",
            "description": "Modifie les propriétés d’un canal patché.",
            "syntaxes": [
                "[1][Patch][Enter]",
                "{Edit}"
            ],
            "examples": [
                "Changer le type de fixture, les paramètres RDM, etc."
            ],
            "tips": [
                "Accès rapide via le bouton {Edit} dans le patch."
            ]
        },
        {
            "id": "patch-clear",
            "subcat": "patch-base",
            "action": "Effacer un patch",
            "description": "Supprime un ou plusieurs canaux du patch.",
            "syntaxes": [
                "[1][Clear][Patch][Enter]",
                "[1][Thru][10][Clear][Patch][Enter]"
            ],
            "examples": [
                "[1][Clear][Patch][Enter] → supprime le patch du canal 1."
            ],
            "tips": [
                "Ne supprime pas les données du show."
            ]
        },
        {
            "id": "fixture-type",
            "subcat": "patch-base",
            "action": "Sélectionner un type de fixture",
            "description": "Choisir un modèle de projecteur dans la bibliothèque.",
            "syntaxes": [
                "[1][Patch][Enter] → {Fixture Type} → [Select]"
            ],
            "examples": [
                "Sélectionner 'Generic Dimmer' ou 'Chauvet ColoRado 1."
            ],
            "tips": [
                "Détermine les paramètres et canaux disponibles."
            ]
        },
        {
            "id": "unpatch",
            "subcat": "patch-base",
            "action": "Dépatcher un canal",
            "description": "Supprime l’adresse DMX d’un canal.",
            "syntaxes": [
                "[1][Unpatch][Enter]"
            ],
            "examples": [
                "[1][Unpatch][Enter] → supprime l’adresse du canal 1."
            ],
            "tips": [
                "Le canal devient non patché mais reste dans la liste."
            ]
        },
        {
            "id": "swap-channels",
            "subcat": "patch-base",
            "action": "Swapper deux canaux",
            "description": "Échange les adresses DMX de deux canaux.",
            "syntaxes": [
                "[1]{Swap}[2][Enter]"
            ],
            "examples": [
                "[1]{Swap}[2][Enter] → échange les adresses de 1 et 2."
            ],
            "tips": [
                "Utile pour réorganiser un patch existant."
            ]
        },
        {
            "id": "device-list",
            "subcat": "patch-base",
            "action": "Utiliser Device List",
            "description": "Affiche la liste des dispositifs patchés.",
            "syntaxes": [
                "{Device List}"
            ],
            "examples": [
                "{Device List} → ouvre la liste des fixtures."
            ],
            "tips": [
                "Utile pour le diagnostic RDM."
            ]
        },
        {
            "id": "group-create",
            "subcat": "groups-base",
            "action": "Créer un Group",
            "description": "Stocke une sélection de canaux pour rappel rapide.",
            "syntaxes": [
                "[1][Thru][10][Group][1][Enter]"
            ],
            "examples": [
                "Group 1 = fronts, Group 2 = backlights."
            ],
            "tips": [
                "Peut contenir des canaux, d'autres groups, ou palettes."
            ]
        },
        {
            "id": "group-recall",
            "subcat": "groups-base",
            "action": "Rappeler un Group",
            "description": "Sélectionne les canaux d’un group.",
            "syntaxes": [
                "[Group][1][Enter]"
            ],
            "examples": [
                "[Group][1][Enter] → sélectionne les canaux du Group 1."
            ],
            "tips": [
                "Utile pour des sélections récurrentes."
            ]
        },
        {
            "id": "palette-create",
            "subcat": "palettes-base",
            "action": "Créer une palette",
            "description": "Stocke une ou plusieurs valeurs (couleur, position, etc.).",
            "syntaxes": [
                "[1][Thru][4][At][Red][At][Pan][Center][Record][Palette][1][Enter]"
            ],
            "examples": [
                "Palette 1 = couleur rouge, position centre."
            ],
            "tips": [
                "Peut contenir des IFCB (Intensity, Focus, Color, Beam)."
            ]
        },
        {
            "id": "palette-apply",
            "subcat": "palettes-base",
            "action": "Appliquer une palette",
            "description": "Applique les valeurs d’une palette à des canaux sélectionnés.",
            "syntaxes": [
                "[1][Thru][4][Palette][1][Enter]"
            ],
            "examples": [
                "Applique la palette 1 aux canaux 1-4."
            ],
            "tips": [
                "Utile pour répéter des looks."
            ]
        },
        {
            "id": "sub-create",
            "subcat": "submasters-base",
            "action": "Créer un Submaster",
            "description": "Crée un contrôle maître pour un ensemble de canaux.",
            "syntaxes": [
                "[1][Thru][10][Sub][101][Enter]"
            ],
            "examples": [
                "Sub 101 = house lights à 100 %."
            ],
            "tips": [
                "Les Submasters sont HTP par défaut."
            ]
        },
        {
            "id": "sub-recall",
            "subcat": "submasters-base",
            "action": "Rappeler un Submaster",
            "description": "Active un submaster avec son niveau.",
            "syntaxes": [
                "[Sub][101][50][Enter]"
            ],
            "examples": [
                "[Sub][101][50][Enter] → active Sub 101 à 50 %."
            ],
            "tips": [
                "Peut être utilisé en [Live] ou [Blind]."
            ]
        },
        {
            "id": "sub-hold",
            "subcat": "submasters-base",
            "action": "Activer le mode Hold",
            "description": "Empêche le Submaster de s’éteindre automatiquement.",
            "syntaxes": [
                "{Hold}[Enter] après l’enregistrement"
            ],
            "examples": [
                "Après avoir enregistré Sub 101, appuyer sur {Hold}."
            ],
            "tips": [
                "Essentiel pour les house lights."
            ]
        },
        {
            "id": "sub-solo",
            "subcat": "submasters-base",
            "action": "Utiliser le mode Solo",
            "description": "Isole un Submaster pour édition.",
            "syntaxes": [
                "[Sub][101]{Solo}[Enter]"
            ],
            "examples": [
                "Seul le Sub 101 est actif."
            ],
            "tips": [
                "À utiliser avec prudence en spectacle."
            ]
        },
        {
            "id": "sub-fade",
            "subcat": "submasters-base",
            "action": "Configurer les temps de fondu d’un Submaster",
            "description": "Définit les temps Up/Down pour un Submaster.",
            "syntaxes": [
                "[Sub][101][Time][2][Time][2][Enter]"
            ],
            "examples": [
                "Sub 101 : Up=2s, Down=2s."
            ],
            "tips": [
                "Visible dans la Submaster List."
            ]
        },
        {
            "id": "sub-exempt",
            "subcat": "submasters-base",
            "action": "Exclure du Grandmaster",
            "description": "Empêche un canal/submaster d’être affecté par le Grandmaster.",
            "syntaxes": [
                "[Sub][101]{Exclude From Grandmaster}[Enter]"
            ],
            "examples": [
                "Les house lights ne sont pas affectées par le GM."
            ],
            "tips": [
                "Option dans les propriétés du Submaster."
            ]
        },
        {
            "id": "cue-record",
            "subcat": "cues-base",
            "action": "Enregistrer un Cue",
            "description": "Stocke l’état actuel dans un cue.",
            "syntaxes": [
                "[1][Thru][10][At][Full][Enter] → [Record][Cue][1][Enter]"
            ],
            "examples": [
                "Cue 1 = fronts à 100 %."
            ],
            "tips": [
                "Par défaut, les données trackent."
            ]
        },
        {
            "id": "cue-go",
            "subcat": "cues-base",
            "action": "Aller à un Cue",
            "description": "Joue un cue existant.",
            "syntaxes": [
                "[Go To Cue][5][Enter]"
            ],
            "examples": [
                "[Go To Cue][1][Enter] pour le premier cue."
            ],
            "tips": [
                "Double pression = [Go To Cue Complete]."
            ]
        },
        {
            "id": "cue-update",
            "subcat": "cues-base",
            "action": "Mettre à jour un Cue",
            "description": "Modifie un cue avec les valeurs actuelles.",
            "syntaxes": [
                "[1][At][50][Enter] → [Update][Cue][1][Enter]"
            ],
            "examples": [
                "Modifier l’intensité du canal 1 dans le Cue 1."
            ],
            "tips": [
                "[Q Only] met à jour uniquement l’intensité."
            ]
        },
        {
            "id": "cue-link",
            "subcat": "cues-base",
            "action": "Lier des Cues",
            "description": "Fait enchaîner automatiquement un cue vers le suivant.",
            "syntaxes": [
                "[Cue][1][Link][2][Enter]"
            ],
            "examples": [
                "Cue 1 joue → Cue 2 joue automatiquement."
            ],
            "tips": [
                "Très utile pour les enchaînements précis."
            ]
        },
        {
            "id": "cue-mark",
            "subcat": "cues-base",
            "action": "Système de Marks (M/m) et AutoMark",
            "description": "Prépare des mouvements non-intensité avant que l’intensité ne revienne.",
            "syntaxes": [
                "[M]",
                "[m]",
                "Setup > AutoMark"
            ],
            "examples": [
                "Positionner les gobos avant le fondu."
            ],
            "tips": [
                "Très utile pour les mouvements."
            ]
        },
        {
            "id": "cue-minus-links",
            "subcat": "cues-base",
            "action": "Utiliser {Minus Links} dans Go To Cue",
            "description": "Exécute un cue sans déclencher ses liens.",
            "syntaxes": [
                "[Go To Cue][4]{Minus Links}[Enter]"
            ],
            "examples": [
                "Joue le cue 4 sans déclencher le cue suivant lié."
            ],
            "tips": [
                "Utile pour tester un cue isolément."
            ]
        },
        {
            "id": "cue-complete",
            "subcat": "cues-base",
            "action": "Utiliser {Complete} dans Go To Cue",
            "description": "Exécute un cue et tous ses liens en chaîne.",
            "syntaxes": [
                "[Go To Cue][4]{Complete}[Enter]"
            ],
            "examples": [
                "Joue le cue 4, puis 5, puis 6 si liés."
            ],
            "tips": [
                "Simule une exécution normale."
            ]
        },
        {
            "id": "cue-out",
            "subcat": "cues-base",
            "action": "Utiliser Go To Cue Out",
            "description": "Réinitialise tous les paramètres et revient au premier cue.",
            "syntaxes": [
                "[Go To Cue][Out][Enter]"
            ],
            "examples": [
                "[Go To Cue][Out] → réinitialise la scène."
            ],
            "tips": [
                "À éviter en représentation."
            ]
        },
        {
            "id": "freeze",
            "subcat": "cues-base",
            "action": "Utiliser [Freeze]",
            "description": "Gèle les valeurs actuelles.",
            "syntaxes": [
                "[Freeze][Enter]"
            ],
            "examples": [
                "[Freeze][Enter] → fige la scène actuelle."
            ],
            "tips": [
                "Utile pour les arrêts d’urgence."
            ]
        },
        {
            "id": "release",
            "subcat": "cues-base",
            "action": "Utiliser [Release]",
            "description": "Masque des données dans un cue.",
            "syntaxes": [
                "[1][Release][Enter]"
            ],
            "examples": [
                "Release l’intensité du canal 1 dans le cue courant."
            ],
            "tips": [
                "Les données restent enregistrées."
            ]
        },
        {
            "id": "release-all",
            "subcat": "cues-base",
            "action": "Utiliser [Release All]",
            "description": "Masque toutes les données manuelles.",
            "syntaxes": [
                "[Release All][Enter]"
            ],
            "examples": [
                "[Release All][Enter] → nettoie la scène."
            ],
            "tips": [
                "Peut être annulé avec [Assert All]."
            ]
        },
        {
            "id": "block",
            "subcat": "cues-base",
            "action": "Utiliser [Block]",
            "description": "Empêche qu’un paramètre soit modifié.",
            "syntaxes": [
                "[1][Block][Enter]"
            ],
            "examples": [
                "Block l’intensité du canal 1 pour qu’elle ne change plus."
            ],
            "tips": [
                "Très puissant mais dangereux si mal utilisé."
            ]
        },
        {
            "id": "trace",
            "subcat": "cues-base",
            "action": "Utiliser [Trace]",
            "description": "Modifie des valeurs dans les cues précédents.",
            "syntaxes": [
                "[1][At][50][Trace][Enter]"
            ],
            "examples": [
                "Ajuste l’intensité dans les 3 derniers cues."
            ],
            "tips": [
                "Respecte les Block."
            ]
        },
        {
            "id": "rem-dim",
            "subcat": "cues-base",
            "action": "Supprimer le tracking (Rem Dim)",
            "description": "Empêche un canal de revenir à son ancien niveau.",
            "syntaxes": [
                "[1][Thru][5][Rem Dim][Enter]"
            ],
            "examples": [
                "Supprime le tracking des canaux 6–10."
            ],
            "tips": [
                "Essentiel pour éviter les intensités fantômes."
            ]
        },
        {
            "id": "preheat",
            "subcat": "cues-base",
            "action": "Utiliser Preheat",
            "description": "Chauffe les filaments avant un fondu montant.",
            "syntaxes": [
                "[1][Preheat][Enter]"
            ],
            "examples": [
                "[1][Preheat][Enter] → active le preheat pour le canal 1."
            ],
            "tips": [
                "Disponible dans le Patch."
            ]
        },
        {
            "id": "record-only",
            "subcat": "cues-base",
            "action": "Utiliser [Record Only]",
            "description": "Enregistre uniquement les données manuelles.",
            "syntaxes": [
                "[Record Only][Cue][3][Enter]",
                "[1][At][50][Enter][Record Only][Cue][4][Enter]"
            ],
            "examples": [
                "[Record Only][Cue][3][Enter] → enregistre uniquement les données manuelles dans le cue 3.",
                "[1][At][50][Enter][Record Only][Cue][4][Enter] → enregistre l'état manuel du canal 1 dans le cue 4."
            ],
            "tips": [
                "Utile pour enregistrer des modifications temporaires.",
                "Peut être combiné avec [Q Only]."
            ]
        },
        {
            "id": "q-only",
            "subcat": "cues-base",
            "action": "Utiliser [Q Only]",
            "description": "Empêche le tracking vers le cue suivant.",
            "syntaxes": [
                "[Record][Cue][5][Q Only][Enter]",
                "[Record Only][Cue][5][Q Only][Enter]"
            ],
            "examples": [
                "[Record][Cue][5][Q Only][Enter] → le cue 5 ne transmet pas ses valeurs au cue 6."
            ],
            "tips": [
                "Utile pour des cues isolés.",
                "Peut être combiné avec [Record Only]."
            ]
        },
        {
            "id": "effect-create",
            "subcat": "effects-base",
            "action": "Créer un Effect",
            "description": "Crée un effet (gobo rotate, dimmer chase, etc.).",
            "syntaxes": [
                "[1][Thru][4][Effect][1][Enter]",
                "[1][Thru][4][Effect][Record][1][Enter]"
            ],
            "examples": [
                "Effect 1 = rotation de gobo sur les canaux 1–4."
            ],
            "tips": [
                "Utiliser le Effect Editor (Tab 32) pour plus de contrôle."
            ]
        },
        {
            "id": "macro-learn",
            "subcat": "macros-base",
            "action": "Enregistrer une Macro avec [Learn]",
            "description": "Capture une séquence de touches.",
            "syntaxes": [
                "[Learn] → [1][Enter][Go To Cue][Out][Enter] → [Learn]"
            ],
            "examples": [
                "Macro 1 = éteint la scène proprement."
            ],
            "tips": [
                "Ne pas utiliser [Clear] pendant l’enregistrement."
            ]
        },
        {
            "id": "macro-background",
            "subcat": "macros-base",
            "action": "Exécuter une macro en arrière-plan",
            "description": "Permet d’exécuter des commandes sans perturber la ligne principale.",
            "syntaxes": [
                "<U0> Macro 1 Enter"
            ],
            "examples": [
                "Bouton 'Clean Sneak' sur Magic Sheet."
            ],
            "tips": [
                "Syntaxe : `<U0>$ Macro [num] Enter`."
            ]
        },
        {
            "id": "macro-wait",
            "subcat": "macros-base",
            "action": "Utiliser {Wait} dans une Macro",
            "description": "Ajoute une pause dans une macro.",
            "syntaxes": [
                "[Learn] → [1][Enter] → {Wait} → [5] → [2][Enter] → [Learn]"
            ],
            "examples": [
                "Macro attend 5 secondes avant d’allumer le canal 2."
            ],
            "tips": [
                "{Wait} en secondes."
            ]
        },
        {
            "id": "macro-loop",
            "subcat": "macros-base",
            "action": "Utiliser {Loop} dans une Macro",
            "description": "Crée une boucle dans une macro.",
            "syntaxes": [
                "[Learn] → {Loop Begin} → [1][At][50][Enter] → {Loop End} → [Loop Num][3][Enter] → [Learn]"
            ],
            "examples": [
                "Macro clignote le canal 1 trois fois."
            ],
            "tips": [
                "Disponible dans le Macro Editor."
            ]
        },
        {
            "id": "ms-create",
            "subcat": "magic-sheets-base",
            "action": "Créer un Magic Sheet",
            "description": "Page personnalisée pour accès rapide.",
            "syntaxes": [
                "[Displays]{Magic Sheet}{+} → Magic Sheet 501[Enter]"
            ],
            "examples": [
                "MS 501 = page d’accueil."
            ],
            "tips": [
                "Utiliser des objets 'Command' pour exécuter des macros."
            ]
        },
        {
            "id": "ms-indicator",
            "subcat": "magic-sheets-base",
            "action": "Créer un indicateur visuel",
            "description": "Affiche visuellement si un Submaster est actif.",
            "syntaxes": [
                "Patch Channel 9101 = MS Indicator",
                "[Blind][Sub][101][Enter] → [9101][At][Full][Enter]"
            ],
            "examples": [
                "Rectangle gris = Sub actif."
            ],
            "tips": [
                "Le canal indicateur ne doit jamais avoir d’adresse DMX."
            ]
        },
        {
            "id": "home-tab",
            "subcat": "magic-sheets-base",
            "action": "Créer une page d'accueil (Home Tab)",
            "description": "Définit un Magic Sheet comme page d'accueil.",
            "syntaxes": [
                "[Displays]{Magic Sheet} → {Options} → {Set as Home Tab}"
            ],
            "examples": [
                "MS 501 devient la page d'accueil."
            ],
            "tips": [
                "Raccourcit l'accès aux commandes fréquentes."
            ]
        },
        {
            "id": "sacn-input",
            "subcat": "show-control-base",
            "action": "sACN Input Monitor",
            "description": "Affiche les valeurs sACN entrantes en temps réel.",
            "syntaxes": [
                "Cible = Address → ex: 8/1"
            ],
            "examples": [
                "Afficher les valeurs RGB(A) d’un univers sACN."
            ],
            "tips": [
                "Les objets sACN se mettent à jour en temps réel."
            ]
        },
        {
            "id": "midi-show-control",
            "subcat": "show-control-base",
            "action": "Utiliser MIDI Show Control",
            "description": "Contrôle Eos via MIDI.",
            "syntaxes": [
                "Go 1",
                "Fire 10"
            ],
            "examples": [
                "Go 1 → joue le cue 1.",
                "Fire 10 → exécute la macro 10."
            ],
            "tips": [
                "Supporte les commandes Go, Stop, Resume, Fire."
            ]
        },
        {
            "id": "osc",
            "subcat": "show-control-base",
            "action": "Utiliser Open Sound Control (OSC)",
            "description": "Contrôle Eos via OSC.",
            "syntaxes": [
                "/eos/cue/1/1/fire"
            ],
            "examples": [
                "/eos/cue/1/1/fire → joue le cue 1.1."
            ],
            "tips": [
                "Supporte l’envoi et la réception de données OSC."
            ]
        },
        {
            "id": "timecode",
            "subcat": "show-control-base",
            "action": "Utiliser le Time Code",
            "description": "Synchronise Eos avec un signal Time Code.",
            "syntaxes": [
                "Setup > Show Control > Time Code → Enable"
            ],
            "examples": [
                "Active le Time Code pour la synchronisation."
            ],
            "tips": [
                "Supporte LTC, MTC, et Art-Net Time Code."
            ]
        },
        {
            "id": "pixel-map",
            "subcat": "pixels-media-base",
            "action": "Mapper des pixels",
            "description": "Configure la disposition physique des pixels.",
            "syntaxes": [
                "Setup > Pixel Mapping"
            ],
            "examples": [
                "Créer une grille 10x10 pour un panneau LED."
            ],
            "tips": [
                "Nécessaire pour contrôler les fixtures pixel-mapped."
            ]
        },
        {
            "id": "channel-check-advanced",
            "subcat": "diagnostics-base",
            "action": "Channel Check avancé avec Next/Last",
            "description": "Parcourt les canaux un par un en Channel Check.",
            "syntaxes": [
                "[1][At][7]<0>{Chan Check}[Enter] → [Next]"
            ],
            "examples": [
                "Passe au canal suivant après vérification."
            ],
            "tips": [
                "Les valeurs reviennent automatiquement à leur état précédent."
            ]
        },
        {
            "id": "query",
            "subcat": "diagnostics-base",
            "action": "Utiliser [Query]",
            "description": "Liste les canaux selon des critères.",
            "syntaxes": [
                "[Query]{Lamp Off}[Enter]"
            ],
            "examples": [
                "Liste tous les projecteurs éteints."
            ],
            "tips": [
                "Très puissant mais méconnu."
            ]
        },
        {
            "id": "about-system",
            "subcat": "diagnostics-base",
            "action": "Utiliser [About] System",
            "description": "Affiche les informations réseau et logicielles.",
            "syntaxes": [
                "[About]{System}"
            ],
            "examples": [
                "[About]{System} → ouvre la liste des consoles connectées."
            ],
            "tips": [
                "Utile pour le multi-console."
            ]
        },
        {
            "id": "copy-to",
            "subcat": "advanced-base",
            "action": "Utiliser [Copy To]",
            "description": "Copie des valeurs entre canaux, groupes ou palettes.",
            "syntaxes": [
                "[1][Copy To][2][Enter]"
            ],
            "examples": [
                "Copier la couleur du canal 1 vers le canal 2."
            ],
            "tips": [
                "Très utile pour uniformiser un système."
            ]
        },
        {
            "id": "recall-from",
            "subcat": "advanced-base",
            "action": "Utiliser [Recall From]",
            "description": "Rappelle des données d’un autre endroit.",
            "syntaxes": [
                "[1][Recall From][Cue][5][Enter]"
            ],
            "examples": [
                "Rappelle l’état du canal 1 dans le cue 5."
            ],
            "tips": [
                "Peut être utilisé avec [Sneak] pour un fondu."
            ]
        },
        {
            "id": "undo",
            "subcat": "advanced-base",
            "action": "Utiliser [Undo]",
            "description": "Annule la dernière commande.",
            "syntaxes": [
                "[Undo]"
            ],
            "examples": [
                "[Undo] après une erreur de [Release]."
            ],
            "tips": [
                "Historique limité."
            ]
        },
        {
            "id": "capture",
            "subcat": "advanced-base",
            "action": "Utiliser [Capture]",
            "description": "Stocke l’état actuel dans un cue ou preset.",
            "syntaxes": [
                "[Capture][Cue][999][Enter]"
            ],
            "examples": [
                "Sauvegarde la scène actuelle dans le cue 999."
            ],
            "tips": [
                "Inclut les données manuelles."
            ]
        },
        {
            "id": "assert",
            "subcat": "emergency-base",
            "action": "Utiliser [Assert]",
            "description": "Force la relecture d’un cue ou submaster.",
            "syntaxes": [
                "[Sub][101][Assert][Enter]"
            ],
            "examples": [
                "Si les lumières ne répondent plus, Assert le Sub House."
            ],
            "tips": [
                "À réserver aux cas de dépannage."
            ]
        },
        {
            "id": "emergency-home",
            "subcat": "emergency-base",
            "action": "Réinitialiser les paramètres non-intensité (Home)",
            "description": "Ramène les projecteurs à leur position de base sans éteindre.",
            "syntaxes": [
                "[Home][Enter]"
            ],
            "examples": [
                "[Home][Enter] pour remettre shutters, zoom, gobo à leur home."
            ],
            "tips": [
                "[Home] ne touche pas l’intensité → sécurisé en pleine scène."
            ]
        },
        {
            "id": "emergency-flash",
            "subcat": "emergency-base",
            "action": "Flash temporaire",
            "description": "Active un canal brièvement tant que la touche est enfoncée.",
            "syntaxes": [
                "[1][Flash]"
            ],
            "examples": [
                "Identifier brièvement le projecteur 1."
            ],
            "tips": [
                "Ne laisse aucune trace dans les données du show."
            ]
        },
        {
            "id": "help-emergency",
            "subcat": "emergency-base",
            "action": "Utiliser le Help interactif",
            "description": "Affiche la fonction, la description et des exemples pour n’importe quelle touche.",
            "syntaxes": [
                "[Help] + [touche]"
            ],
            "examples": [
                "[Help] + [Sub] affiche la syntaxe de création de Submaster."
            ],
            "tips": [
                "Fonctionne aussi sur les softkeys et objets Magic Sheet."
            ]
        },
        {
            "id": "allfade",
            "subcat": "emergency-base",
            "action": "Utiliser [Allfade]",
            "description": "Fait fondre progressivement toute la scène à 0 %.",
            "syntaxes": [
                "[Allfade][Enter]"
            ],
            "examples": [
                "[Allfade][Enter] → fin de spectacle douce."
            ],
            "tips": [
                "Le Allfade Master se configure dans Setup > Timings."
            ]
        },
        {
            "id": "grandmaster",
            "subcat": "emergency-base",
            "action": "Utiliser le Grandmaster",
            "description": "Contrôle global de l’intensité.",
            "syntaxes": [
                "[Grandmaster][50][Enter]"
            ],
            "examples": [
                "[Grandmaster][50][Enter] → met le GM à 50 %."
            ],
            "tips": [
                "Les Submasters peuvent être exclus du GM."
            ]
        },
        {
            "id": "file-save",
            "subcat": "file-base",
            "action": "Sauvegarder un fichier show",
            "description": "Enregistre le show dans la mémoire interne ou sur clé USB.",
            "syntaxes": [
                "[File][Save][Enter]"
            ],
            "examples": [
                "[File][Save][Enter] → sauvegarde le show actuel."
            ],
            "tips": [
                "Utiliser [Save As] pour créer une copie."
            ]
        },
        {
            "id": "file-open",
            "subcat": "file-base",
            "action": "Ouvrir un fichier show",
            "description": "Charge un show depuis la mémoire interne ou une clé USB.",
            "syntaxes": [
                "[File][Open][Enter]"
            ],
            "examples": [
                "[File][Open][Show File Archive][1][Select] → ouvre le show 1."
            ],
            "tips": [
                "Toujours sauvegarder avant d'ouvrir un nouveau show."
            ]
        },
        {
            "id": "quick-save",
            "subcat": "file-base",
            "action": "Quick Save (Shift + Update)",
            "description": "Sauvegarde rapidement le show dans l’archive interne.",
            "syntaxes": [
                "[Shift][Update]"
            ],
            "examples": [
                "Appuyer [Shift][Update] à la fin de chaque session."
            ],
            "tips": [
                "Sauvegarde automatiquement dans Show File Archive."
            ]
        },
        {
            "id": "clear-functions",
            "subcat": "file-base",
            "action": "Utiliser les fonctions Clear",
            "description": "Supprime des parties du show.",
            "syntaxes": [
                "[Clear]{Clear Show}[Enter]"
            ],
            "examples": [
                "[Clear]{Clear Show}[Enter] → efface tout le show."
            ],
            "tips": [
                "À utiliser avec prudence."
            ]
        },
        {
            "id": "import-show",
            "subcat": "file-base",
            "action": "Importer un fichier show",
            "description": "Importe un show depuis une clé USB ou le réseau.",
            "syntaxes": [
                "[File][Open][Show File Archive][1][Select]"
            ],
            "examples": [
                "Ouvre le show 1 depuis l’archive."
            ],
            "tips": [
                "Supporte les formats ESF, ESF2, ESF3."
            ]
        },
        {
            "id": "partition-select",
            "subcat": "partition-base",
            "action": "Sélectionner une Partition",
            "description": "Passe à une partition spécifique.",
            "syntaxes": [
                "{Partition}[1][Enter]"
            ],
            "examples": [
                "{Partition}[1][Enter] → active la partition 1."
            ],
            "tips": [
                "Partition 901 = accès à tous les canaux."
            ]
        }
    ]
}
"#;

// --- Composant principal ---
#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| {
        let stored: Option<InitialData> = LocalStorage::get("eos_guide_state").ok();
        stored.unwrap_or_else(|| {
            let initial: InitialData = serde_json::from_str(INITIAL_DATA_JSON).unwrap();
            // Sauvegarder les données initiales si elles ne sont pas déjà présentes
            if let Err(e) = LocalStorage::set("eos_guide_state", &initial) {
                console::error_1(&format!("Erreur sauvegarde initiale: {:?}", e).into());
            }
            initial
        })
    });

    let current_view = use_state(|| View::Home);
    let current_category_id = use_state(|| None::<String>);
    let current_subcategory_id = use_state(|| None::<String>);
    let current_command_id = use_state(|| None::<String>);

    let state_clone = state.clone();
    let save_state = Callback::from(move |new_state: InitialData| {
        state_clone.set(new_state.clone());
        if let Err(e) = LocalStorage::set("eos_guide_state", &new_state) {
            console::error_1(&format!("Erreur de sauvegarde: {:?}", e).into());
        }
    });

    // --- Callbacks de navigation ---
    let go_back = {
        let current_view = current_view.clone();
        let current_category_id = current_category_id.clone();
        let current_subcategory_id = current_subcategory_id.clone();
        let current_command_id = current_command_id.clone();
        Callback::from(move |_| {
            match *current_view {
                View::Detail => {
                    current_view.set(View::Commands);
                    current_command_id.set(None);
                }
                View::Commands => {
                    current_view.set(View::Subcategories);
                    current_subcategory_id.set(None);
                }
                View::Subcategories => {
                    current_view.set(View::Home);
                    current_category_id.set(None);
                }
                View::Home => {} // Déjà à la racine
            }
        })
    };

    let show_home = {
        let current_view = current_view.clone();
        let current_category_id = current_category_id.clone();
        let current_subcategory_id = current_subcategory_id.clone();
        let current_command_id = current_command_id.clone();
        Callback::from(move |_| {
            current_view.set(View::Home);
            current_category_id.set(None);
            current_subcategory_id.set(None);
            current_command_id.set(None);
        })
    };

    let show_subcategories = {
        let current_view = current_view.clone();
        let current_category_id = current_category_id.clone();
        Callback::from(move |cat_id: String| {
            current_view.set(View::Subcategories);
            current_category_id.set(Some(cat_id));
        })
    };

    let show_commands = {
        let current_view = current_view.clone();
        let current_subcategory_id = current_subcategory_id.clone();
        Callback::from(move |subcat_id: String| {
            current_view.set(View::Commands);
            current_subcategory_id.set(Some(subcat_id));
        })
    };

    let show_detail = {
        let current_view = current_view.clone();
        let current_command_id = current_command_id.clone();
        Callback::from(move |cmd_id: String| {
            current_view.set(View::Detail);
            current_command_id.set(Some(cmd_id));
        })
    };

    // --- Callbacks de modification ---
    let add_command = {
        let state = state.clone();
        let save_state = save_state.clone();
        let current_subcategory_id = current_subcategory_id.clone();
        Callback::from(move |new_cmd: Command| {
            let mut new_state = (*state).clone();
            let subcat_id = current_subcategory_id.as_deref().unwrap_or_default(); // Devrait être défini
            if new_cmd.subcat == subcat_id { // Vérifier que le subcat est correct
                 new_state.commands.push(new_cmd);
                 save_state.emit(new_state);
            } else {
                 // Gérer erreur ou ignorer
                 log!("Erreur: subcat de la commande ne correspond pas à la sous-catégorie courante.");
            }
        })
    };

    let edit_command = {
        let state = state.clone();
        let save_state = save_state.clone();
        Callback::from(move |updated_cmd: Command| {
            let mut new_state = (*state).clone();
            if let Some(index) = new_state.commands.iter().position(|c| c.id == updated_cmd.id) {
                new_state.commands[index] = updated_cmd;
                save_state.emit(new_state);
            }
        })
    };

    let delete_command = {
        let state = state.clone();
        let save_state = save_state.clone();
        Callback::from(move |cmd_id: String| {
            let mut new_state = (*state).clone();
            new_state.commands.retain(|c| c.id != cmd_id);
            save_state.emit(new_state);
        })
    };

    // --- Rendu conditionnel selon la vue ---
    let view_html = match *current_view {
        View::Home => html! {
            <HomeView
                state={(*state).clone()}
                on_show_subcategories={show_subcategories}
            />
        },
        View::Subcategories => {
            let cat_id = current_category_id.as_deref().unwrap_or_default();
            html! {
                <SubcategoriesView
                    state={(*state).clone()}
                    category_id={cat_id.to_string()}
                    on_show_commands={show_commands}
                />
            }
        }
        View::Commands => {
            let subcat_id = current_subcategory_id.as_deref().unwrap_or_default();
            html! {
                <CommandsView
                    state={(*state).clone()}
                    subcategory_id={subcat_id.to_string()}
                    on_show_detail={show_detail}
                    on_add_command={add_command}
                />
            }
        }
        View::Detail => {
            let cmd_id = current_command_id.as_deref().unwrap_or_default();
            html! {
                <DetailView
                    state={(*state).clone()}
                    command_id={cmd_id.to_string()}
                    on_edit={edit_command}
                    on_delete={delete_command}
                    on_go_back={go_back.clone()}
                />
            }
        }
    };

    html! {
        <div class="container">
            <Header on_show_home={show_home} on_go_back={go_back} current_view={(*current_view).clone()} />
            {view_html}
        </div>
    }
}

// --- Types pour la navigation ---
#[derive(Clone, PartialEq)]
enum View {
    Home,
    Subcategories,
    Commands,
    Detail,
}

// --- Composant d'en-tête ---
#[derive(Properties, PartialEq, Clone)]
struct HeaderProps {
    on_show_home: Callback<MouseEvent>,
    on_go_back: Callback<MouseEvent>,
    current_view: View,
}

#[function_component(Header)]
fn header(props: &HeaderProps) -> Html {
    let show_back_button = props.current_view != View::Home;

    html! {
        <div class="header">
            if show_back_button {
                <button class="back-btn" onclick={props.on_go_back.clone()}>{"← Retour"}</button>
            }
            <h1>{ "📘 Guide Eos" }</h1>
            <button class="back-btn" onclick={props.on_show_home.clone()}>{"?"}</button>
        </div>
    }
}

// --- Composant Vue Home ---
#[derive(Properties, PartialEq)]
struct HomeViewProps {
    state: InitialData,
    on_show_subcategories: Callback<String>,
}

#[function_component(HomeView)]
fn home_view(props: &HomeViewProps) -> Html {
    html! {
        <div id="home-view" class="view active">
            <div class="category-grid">
                { for props.state.categories.iter().map(|cat| {
                    html! {
                        <div class="category-card" onclick={Callback::from({
                            let on_show_subcategories = props.on_show_subcategories.clone();
                            let cat_id = cat.id.clone();
                            move |_| on_show_subcategories.emit(cat_id.clone())
                        })}>
                            <h3>{ &cat.icon }{ " " }{ &cat.name }</h3>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

// --- Composant Vue Sous-catégories ---
#[derive(Properties, PartialEq)]
struct SubcategoriesViewProps {
    state: InitialData,
    category_id: String,
    on_show_commands: Callback<String>,
}

#[function_component(SubcategoriesView)]
fn subcategories_view(props: &SubcategoriesViewProps) -> Html {
    let category = props.state.categories.iter().find(|c| c.id == props.category_id);
    let subcategories: Vec<&Subcategory> = props
        .state
        .subcategories
        .iter()
        .filter(|sc| sc.parent_id == props.category_id)
        .collect();

    if let Some(cat) = category {
        html! {
            <div id="subcategory-view" class="view active">
                <h1>{ &cat.name }</h1>
                <ul class="subcategory-list">
                    { for subcategories.iter().map(|sc| {
                        html! {
                            <li class="subcategory-item" onclick={Callback::from({
                                let on_show_commands = props.on_show_commands.clone();
                                let sc_id = sc.id.clone();
                                move |_| on_show_commands.emit(sc_id.clone())
                            })}>
                                <h3 class="subcategory-title">{ &sc.name }</h3>
                            </li>
                        }
                    })}
                </ul>
            </div>
        }
    } else {
        html! { <div>{"Catégorie introuvable"}</div> }
    }
}

// --- Composant Vue Commandes ---
#[derive(Properties, PartialEq)]
struct CommandsViewProps {
    state: InitialData,
    subcategory_id: String,
    on_show_detail: Callback<String>,
    on_add_command: Callback<Command>,
}

#[function_component(CommandsView)]
fn commands_view(props: &CommandsViewProps) -> Html {
    let subcategory = props.state.subcategories.iter().find(|sc| sc.id == props.subcategory_id);
    let category = subcategory.and_then(|sc| props.state.categories.iter().find(|c| c.id == sc.parent_id));
    let commands: Vec<&Command> = props
        .state
        .commands
        .iter()
        .filter(|c| c.subcat == props.subcategory_id)
        .collect();

    if let Some(cat) = category {
        if let Some(subcat) = subcategory {
            // --- CORRECTION : Capturer les valeurs nécessaires pour la closure ---
            let on_add_command = props.on_add_command.clone();
            let subcat_id_for_closure = props.subcategory_id.clone();
            html! {
                <div id="command-list-view" class="view active">
                    <h1>{ format!("{} > {}", cat.name, subcat.name) }</h1>
                    <button class="back-btn" onclick={
                        Callback::from(move |_| {
                            // Ouvrir une modale ou un composant pour ajouter une commande
                            // Ici, on émet une commande vide ou un signal pour ouvrir la modale
                            let new_cmd = Command {
                                id: "new".to_string(), // Générer un ID unique réellement
                                subcat: subcat_id_for_closure.clone(), // Utiliser la valeur capturée
                                action: "".to_string(),
                                description: "".to_string(),
                                syntaxes: None,
                                examples: None,
                                tips: None,
                            };
                            on_add_command.emit(new_cmd); // Utiliser le callback cloné
                        })
                    }>{"➕ Ajouter une commande"}</button>
                    <div id="command-list-container">
                        { for commands.iter().map(|cmd| {
                            html! {
                                <div class="command-item" onclick={Callback::from({
                                    let on_show_detail = props.on_show_detail.clone();
                                    let cmd_id = cmd.id.clone();
                                    move |_| on_show_detail.emit(cmd_id.clone())
                                })}>
                                    <h3 class="command-title">{ &cmd.action }</h3>
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
        } else {
            html! { <div>{"Sous-catégorie introuvable"}</div> }
        }
    } else {
        html! { <div>{"Catégorie parente introuvable"}</div> }
    }
}


// --- Composant Vue Détail ---
#[derive(Properties, PartialEq)]
struct DetailViewProps {
    state: InitialData,
    command_id: String,
    on_edit: Callback<Command>,
    on_delete: Callback<String>,
    on_go_back: Callback<MouseEvent>,
}

#[function_component(DetailView)]
fn detail_view(props: &DetailViewProps) -> Html {
    let command = props.state.commands.iter().find(|c| c.id == props.command_id);

    if let Some(cmd) = command {
        let syntax_html = cmd.syntaxes.as_ref().map(|s| {
            s.iter().map(|s| {
                let processed = s.replace("[", "<span class=\"key key-hard\">[")
                                 .replace("]", "]</span>")
                                 .replace("{", "<span class=\"key key-soft\">{")
                                 .replace("}", "}</span>")
                                 .replace("«", "<span class=\"key key-user\">«")
                                 .replace("»", "»</span>")
                                 .replace("<", "<span class=\"key key-user\">&lt;")
                                 .replace(">", "&gt;</span>");
                html! { <div class="syntax" dangerously_set_inner_html={processed.clone()}></div> }
            }).collect::<Html>()
        }).unwrap_or_default();

        // --- CORRECTION : Capturer les valeurs nécessaires pour les closures ---
        let cmd_id_for_delete = cmd.id.clone();
        let on_delete = props.on_delete.clone();
        let cmd_for_edit = cmd.clone();
        let on_edit = props.on_edit.clone();

        html! {
            <div id="detail-view" class="view active">
                <h1>{ &cmd.action }</h1>
                <div class="detail-card">
                    <p>{ &cmd.description }</p>
                    { syntax_html }
                    if let Some(examples) = &cmd.examples {
                        <div class="examples">
                            <h4>{ "Exemples :" }</h4>
                            <ul>
                                { for examples.iter().map(|e| html! { <li>{ e }</li> }) }
                            </ul>
                        </div>
                    }
                    if let Some(tips) = &cmd.tips {
                        <div class="tips">
                            <h4>{ "Conseils & subtilités :" }</h4>
                            <ul>
                                { for tips.iter().map(|t| html! { <li>{ t }</li> }) }
                            </ul>
                        </div>
                    }
                    <div class="action-buttons">
                        <button onclick={
                            Callback::from(move |_| {
                                // Ouvrir une modale d'édition avec les données de `cmd`
                                // Ici, on émet la commande actuelle pour modification
                                on_edit.emit(cmd_for_edit.clone()); // Utiliser la commande clonée
                            })
                        }>{ "✏️ Éditer" }</button>
                        <button onclick={
                            Callback::from(move |_| {
                                if web_sys::window().unwrap().confirm_with_message("Êtes-vous sûr de vouloir supprimer cette commande ?").unwrap_or(false) {
                                    on_delete.emit(cmd_id_for_delete.clone()); // Utiliser l'ID cloné
                                }
                            })
                        }>{ "🗑️ Supprimer" }</button>
                    </div>
                </div>
            </div>
        }
    } else {
        html! { <div>{"Commande introuvable"}</div> }
    }
}

// --- CSS (à inclure dans index.html ou via trunk) ---
const CSS: &str = r#"
:root {
    --bg: #1e1e1e;
    --text: #ffffff;
    --card-bg: #2d2d2d;
    --card-shadow: 0 2px 8px rgba(0,0,0,0.4);
    --syntax-bg: #333333;
    --border: #444444;
    --header: #ff6b6b;
    --section-header: #4ecdc4;
    --tips: #a5d6a7;
    --btn-bg: #333333;
    --btn-text: #ffffff;
    --btn-hover: #444444;
    --key-hard: #1a1a1a;
    --key-soft: #252525;
    --key-user: #3a3a3a;
    --key-border: #555555;
}
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: var(--bg); color: var(--text); line-height: 1.6; }
.container { max-width: 800px; margin: 0 auto; padding: 20px; }
.view { display: none; }
.view.active { display: block; }
h1, h2, h3, h4 { margin-bottom: 12px; color: var(--header); }
h2 { color: var(--section-header); }
p, ul, ol { margin-bottom: 12px; }
ul, ol { padding-left: 20px; }
li { margin-bottom: 6px; }
a { color: #4ecdc4; text-decoration: none; }
a:hover { text-decoration: underline; }
.header { background: rgba(0,0,0,0.3); padding: 15px; border-radius: 8px; margin-bottom: 20px; display: flex; justify-content: space-between; align-items: center; }
.back-btn { background: var(--btn-bg); color: var(--btn-text); border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-weight: 600; display: inline-flex; align-items: center; gap: 6px; }
.back-btn:hover { background: var(--btn-hover); }
.category-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 16px; }
.category-card { background: var(--card-bg); padding: 20px; border-radius: 8px; box-shadow: var(--card-shadow); border: 1px solid var(--border); text-align: center; cursor: pointer; transition: transform 0.2s; }
.category-card:hover { transform: translateY(-3px); }
.category-card h3 { font-size: 1.2em; display: flex; align-items: center; justify-content: center; gap: 8px; }
.subcategory-list, .command-list { list-style: none; }
.subcategory-item, .command-item { background: var(--card-bg); margin-bottom: 12px; padding: 16px; border-radius: 8px; box-shadow: var(--card-shadow); border: 1px solid var(--border); cursor: pointer; transition: background 0.2s; }
.subcategory-item:hover, .command-item:hover { background: #3a3a3a; }
.subcategory-title, .command-title { font-weight: 600; }
.syntax { background: var(--syntax-bg); padding: 12px; border-radius: 6px; margin: 12px 0; font-family: monospace; white-space: pre-wrap; overflow-x: auto; }
.examples, .tips { margin: 12px 0; }
.examples ul, .tips ul { list-style-type: none; padding-left: 0; }
.examples li, .tips li { background: rgba(78, 205, 196, 0.1); padding: 8px; margin-bottom: 6px; border-radius: 4px; border-left: 3px solid var(--section-header); }
.tips li { background: rgba(165, 214, 167, 0.1); border-left-color: var(--tips); }
.key { display: inline-block; background: var(--key-hard); border: 1px solid var(--key-border); border-radius: 4px; padding: 2px 6px; margin: 0 2px; font-family: monospace; font-size: 0.9em; min-width: 24px; text-align: center; }
.key-soft { background: var(--key-soft); }
.key-user { background: var(--key-user); }
.legend { margin-top: 20px; padding: 12px; background: var(--card-bg); border-radius: 6px; font-size: 0.9em; }
.legend ul { list-style-type: none; padding-left: 0; }
.legend li { margin-bottom: 8px; display: flex; align-items: flex-start; gap: 8px; }
.legend .key { align-self: flex-start; margin-top: 2px; }
.action-buttons { margin-top: 20px; display: flex; gap: 12px; justify-content: center; }
.action-buttons button { padding: 8px 16px; font-weight: 600; }
.modal { position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.75); display: flex; justify-content: center; align-items: center; z-index: 1000; opacity: 0; visibility: hidden; transition: opacity 0.2s, visibility 0.2s; }
.modal.active { opacity: 1; visibility: visible; }
.modal-content { background: var(--card-bg); padding: 20px; border-radius: 8px; width: 90%; max-width: 600px; box-shadow: 0 4px 20px rgba(0,0,0,0.5); max-height: 80vh; overflow-y: auto; }
.modal-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px; }
.modal-title { font-size: 1.4em; font-weight: bold; }
.close-btn { background: none; border: none; color: var(--text); font-size: 1.5em; cursor: pointer; }
.form-group { margin-bottom: 15px; }
.form-group label { display: block; margin-bottom: 5px; font-weight: 600; }
.form-group input, .form-group textarea, .form-group select { width: 100%; padding: 8px; border-radius: 4px; border: 1px solid var(--border); background: var(--bg); color: var(--text); }
.form-actions { display: flex; gap: 10px; justify-content: flex-end; }
.form-actions button { padding: 8px 16px; font-weight: 600; }
.footer { text-align: center; margin-top: 30px; padding-top: 20px; border-top: 1px solid var(--border); font-size: 0.8em; color: #aaa; }
/* Responsive */
@media (max-width: 600px) {
    .category-grid { grid-template-columns: 1fr; }
    .header { flex-direction: column; gap: 10px; }
    .action-buttons { flex-direction: column; }
    .action-buttons button { width: 100%; }
}
"#;

fn main() {
    yew::Renderer::<App>::new().render();
}