/// Country code data module for phone prefix selection with SVG flag icon support.
use std::path::PathBuf;

/// Holds static information about a country for phone prefix selection.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct CountryInfo {
    /// ISO 3166-1 alpha-2 country code (e.g., "de", "us").
    pub iso_code: &'static str,
    /// German name of the country (e.g., "Deutschland").
    pub name_de: &'static str,
    /// English name of the country (e.g., "Germany").
    pub name_en: &'static str,
    /// International calling code including the "+" prefix (e.g., "+49").
    pub calling_code: &'static str,
}

/// Returns a static slice of all country entries, sorted alphabetically by `name_de`.
///
/// Includes all 193 UN member states plus Vatican City, Palestine, Taiwan,
/// Hong Kong, and Kosovo (197 entries total).
///
/// German sorting: ä→a, ö→o, ü→u (DIN 5007-1 dictionary order).
pub fn countries() -> &'static [CountryInfo] {
    &[
        // ═══ A ═══
        CountryInfo {
            iso_code: "af",
            name_de: "Afghanistan",
            name_en: "Afghanistan",
            calling_code: "+93",
        },
        CountryInfo {
            iso_code: "eg",
            name_de: "Ägypten",
            name_en: "Egypt",
            calling_code: "+20",
        },
        CountryInfo {
            iso_code: "al",
            name_de: "Albanien",
            name_en: "Albania",
            calling_code: "+355",
        },
        CountryInfo {
            iso_code: "dz",
            name_de: "Algerien",
            name_en: "Algeria",
            calling_code: "+213",
        },
        CountryInfo {
            iso_code: "ad",
            name_de: "Andorra",
            name_en: "Andorra",
            calling_code: "+376",
        },
        CountryInfo {
            iso_code: "ao",
            name_de: "Angola",
            name_en: "Angola",
            calling_code: "+244",
        },
        CountryInfo {
            iso_code: "ag",
            name_de: "Antigua und Barbuda",
            name_en: "Antigua and Barbuda",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "gq",
            name_de: "Äquatorialguinea",
            name_en: "Equatorial Guinea",
            calling_code: "+240",
        },
        CountryInfo {
            iso_code: "ar",
            name_de: "Argentinien",
            name_en: "Argentina",
            calling_code: "+54",
        },
        CountryInfo {
            iso_code: "am",
            name_de: "Armenien",
            name_en: "Armenia",
            calling_code: "+374",
        },
        CountryInfo {
            iso_code: "az",
            name_de: "Aserbaidschan",
            name_en: "Azerbaijan",
            calling_code: "+994",
        },
        CountryInfo {
            iso_code: "et",
            name_de: "Äthiopien",
            name_en: "Ethiopia",
            calling_code: "+251",
        },
        CountryInfo {
            iso_code: "au",
            name_de: "Australien",
            name_en: "Australia",
            calling_code: "+61",
        },
        // ═══ B ═══
        CountryInfo {
            iso_code: "bs",
            name_de: "Bahamas",
            name_en: "Bahamas",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "bh",
            name_de: "Bahrain",
            name_en: "Bahrain",
            calling_code: "+973",
        },
        CountryInfo {
            iso_code: "bd",
            name_de: "Bangladesch",
            name_en: "Bangladesh",
            calling_code: "+880",
        },
        CountryInfo {
            iso_code: "bb",
            name_de: "Barbados",
            name_en: "Barbados",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "by",
            name_de: "Belarus",
            name_en: "Belarus",
            calling_code: "+375",
        },
        CountryInfo {
            iso_code: "be",
            name_de: "Belgien",
            name_en: "Belgium",
            calling_code: "+32",
        },
        CountryInfo {
            iso_code: "bz",
            name_de: "Belize",
            name_en: "Belize",
            calling_code: "+501",
        },
        CountryInfo {
            iso_code: "bj",
            name_de: "Benin",
            name_en: "Benin",
            calling_code: "+229",
        },
        CountryInfo {
            iso_code: "bt",
            name_de: "Bhutan",
            name_en: "Bhutan",
            calling_code: "+975",
        },
        CountryInfo {
            iso_code: "bo",
            name_de: "Bolivien",
            name_en: "Bolivia",
            calling_code: "+591",
        },
        CountryInfo {
            iso_code: "ba",
            name_de: "Bosnien und Herzegowina",
            name_en: "Bosnia and Herzegovina",
            calling_code: "+387",
        },
        CountryInfo {
            iso_code: "bw",
            name_de: "Botsuana",
            name_en: "Botswana",
            calling_code: "+267",
        },
        CountryInfo {
            iso_code: "br",
            name_de: "Brasilien",
            name_en: "Brazil",
            calling_code: "+55",
        },
        CountryInfo {
            iso_code: "bn",
            name_de: "Brunei",
            name_en: "Brunei",
            calling_code: "+673",
        },
        CountryInfo {
            iso_code: "bg",
            name_de: "Bulgarien",
            name_en: "Bulgaria",
            calling_code: "+359",
        },
        CountryInfo {
            iso_code: "bf",
            name_de: "Burkina Faso",
            name_en: "Burkina Faso",
            calling_code: "+226",
        },
        CountryInfo {
            iso_code: "bi",
            name_de: "Burundi",
            name_en: "Burundi",
            calling_code: "+257",
        },
        // ═══ C ═══
        CountryInfo {
            iso_code: "cl",
            name_de: "Chile",
            name_en: "Chile",
            calling_code: "+56",
        },
        CountryInfo {
            iso_code: "cn",
            name_de: "China",
            name_en: "China",
            calling_code: "+86",
        },
        CountryInfo {
            iso_code: "cr",
            name_de: "Costa Rica",
            name_en: "Costa Rica",
            calling_code: "+506",
        },
        // ═══ D ═══
        CountryInfo {
            iso_code: "dk",
            name_de: "Dänemark",
            name_en: "Denmark",
            calling_code: "+45",
        },
        CountryInfo {
            iso_code: "de",
            name_de: "Deutschland",
            name_en: "Germany",
            calling_code: "+49",
        },
        CountryInfo {
            iso_code: "dm",
            name_de: "Dominica",
            name_en: "Dominica",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "do",
            name_de: "Dominikanische Republik",
            name_en: "Dominican Republic",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "dj",
            name_de: "Dschibuti",
            name_en: "Djibouti",
            calling_code: "+253",
        },
        // ═══ E ═══
        CountryInfo {
            iso_code: "ec",
            name_de: "Ecuador",
            name_en: "Ecuador",
            calling_code: "+593",
        },
        CountryInfo {
            iso_code: "sv",
            name_de: "El Salvador",
            name_en: "El Salvador",
            calling_code: "+503",
        },
        CountryInfo {
            iso_code: "ci",
            name_de: "Elfenbeinküste",
            name_en: "Ivory Coast",
            calling_code: "+225",
        },
        CountryInfo {
            iso_code: "er",
            name_de: "Eritrea",
            name_en: "Eritrea",
            calling_code: "+291",
        },
        CountryInfo {
            iso_code: "ee",
            name_de: "Estland",
            name_en: "Estonia",
            calling_code: "+372",
        },
        CountryInfo {
            iso_code: "sz",
            name_de: "Eswatini",
            name_en: "Eswatini",
            calling_code: "+268",
        },
        // ═══ F ═══
        CountryInfo {
            iso_code: "fj",
            name_de: "Fidschi",
            name_en: "Fiji",
            calling_code: "+679",
        },
        CountryInfo {
            iso_code: "fi",
            name_de: "Finnland",
            name_en: "Finland",
            calling_code: "+358",
        },
        CountryInfo {
            iso_code: "fr",
            name_de: "Frankreich",
            name_en: "France",
            calling_code: "+33",
        },
        // ═══ G ═══
        CountryInfo {
            iso_code: "ga",
            name_de: "Gabun",
            name_en: "Gabon",
            calling_code: "+241",
        },
        CountryInfo {
            iso_code: "gm",
            name_de: "Gambia",
            name_en: "Gambia",
            calling_code: "+220",
        },
        CountryInfo {
            iso_code: "ge",
            name_de: "Georgien",
            name_en: "Georgia",
            calling_code: "+995",
        },
        CountryInfo {
            iso_code: "gh",
            name_de: "Ghana",
            name_en: "Ghana",
            calling_code: "+233",
        },
        CountryInfo {
            iso_code: "gd",
            name_de: "Grenada",
            name_en: "Grenada",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "gr",
            name_de: "Griechenland",
            name_en: "Greece",
            calling_code: "+30",
        },
        CountryInfo {
            iso_code: "gt",
            name_de: "Guatemala",
            name_en: "Guatemala",
            calling_code: "+502",
        },
        CountryInfo {
            iso_code: "gn",
            name_de: "Guinea",
            name_en: "Guinea",
            calling_code: "+224",
        },
        CountryInfo {
            iso_code: "gw",
            name_de: "Guinea-Bissau",
            name_en: "Guinea-Bissau",
            calling_code: "+245",
        },
        CountryInfo {
            iso_code: "gy",
            name_de: "Guyana",
            name_en: "Guyana",
            calling_code: "+592",
        },
        // ═══ H ═══
        CountryInfo {
            iso_code: "ht",
            name_de: "Haiti",
            name_en: "Haiti",
            calling_code: "+509",
        },
        CountryInfo {
            iso_code: "hn",
            name_de: "Honduras",
            name_en: "Honduras",
            calling_code: "+504",
        },
        CountryInfo {
            iso_code: "hk",
            name_de: "Hongkong",
            name_en: "Hong Kong",
            calling_code: "+852",
        },
        // ═══ I ═══
        CountryInfo {
            iso_code: "in",
            name_de: "Indien",
            name_en: "India",
            calling_code: "+91",
        },
        CountryInfo {
            iso_code: "id",
            name_de: "Indonesien",
            name_en: "Indonesia",
            calling_code: "+62",
        },
        CountryInfo {
            iso_code: "iq",
            name_de: "Irak",
            name_en: "Iraq",
            calling_code: "+964",
        },
        CountryInfo {
            iso_code: "ir",
            name_de: "Iran",
            name_en: "Iran",
            calling_code: "+98",
        },
        CountryInfo {
            iso_code: "ie",
            name_de: "Irland",
            name_en: "Ireland",
            calling_code: "+353",
        },
        CountryInfo {
            iso_code: "is",
            name_de: "Island",
            name_en: "Iceland",
            calling_code: "+354",
        },
        CountryInfo {
            iso_code: "il",
            name_de: "Israel",
            name_en: "Israel",
            calling_code: "+972",
        },
        CountryInfo {
            iso_code: "it",
            name_de: "Italien",
            name_en: "Italy",
            calling_code: "+39",
        },
        // ═══ J ═══
        CountryInfo {
            iso_code: "jm",
            name_de: "Jamaika",
            name_en: "Jamaica",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "jp",
            name_de: "Japan",
            name_en: "Japan",
            calling_code: "+81",
        },
        CountryInfo {
            iso_code: "ye",
            name_de: "Jemen",
            name_en: "Yemen",
            calling_code: "+967",
        },
        CountryInfo {
            iso_code: "jo",
            name_de: "Jordanien",
            name_en: "Jordan",
            calling_code: "+962",
        },
        // ═══ K ═══
        CountryInfo {
            iso_code: "kh",
            name_de: "Kambodscha",
            name_en: "Cambodia",
            calling_code: "+855",
        },
        CountryInfo {
            iso_code: "cm",
            name_de: "Kamerun",
            name_en: "Cameroon",
            calling_code: "+237",
        },
        CountryInfo {
            iso_code: "ca",
            name_de: "Kanada",
            name_en: "Canada",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "cv",
            name_de: "Kap Verde",
            name_en: "Cape Verde",
            calling_code: "+238",
        },
        CountryInfo {
            iso_code: "kz",
            name_de: "Kasachstan",
            name_en: "Kazakhstan",
            calling_code: "+7",
        },
        CountryInfo {
            iso_code: "qa",
            name_de: "Katar",
            name_en: "Qatar",
            calling_code: "+974",
        },
        CountryInfo {
            iso_code: "ke",
            name_de: "Kenia",
            name_en: "Kenya",
            calling_code: "+254",
        },
        CountryInfo {
            iso_code: "kg",
            name_de: "Kirgisistan",
            name_en: "Kyrgyzstan",
            calling_code: "+996",
        },
        CountryInfo {
            iso_code: "ki",
            name_de: "Kiribati",
            name_en: "Kiribati",
            calling_code: "+686",
        },
        CountryInfo {
            iso_code: "co",
            name_de: "Kolumbien",
            name_en: "Colombia",
            calling_code: "+57",
        },
        CountryInfo {
            iso_code: "km",
            name_de: "Komoren",
            name_en: "Comoros",
            calling_code: "+269",
        },
        CountryInfo {
            iso_code: "cd",
            name_de: "Kongo (Demokratische Republik)",
            name_en: "DR Congo",
            calling_code: "+243",
        },
        CountryInfo {
            iso_code: "cg",
            name_de: "Kongo (Republik)",
            name_en: "Republic of the Congo",
            calling_code: "+242",
        },
        CountryInfo {
            iso_code: "xk",
            name_de: "Kosovo",
            name_en: "Kosovo",
            calling_code: "+383",
        },
        CountryInfo {
            iso_code: "hr",
            name_de: "Kroatien",
            name_en: "Croatia",
            calling_code: "+385",
        },
        CountryInfo {
            iso_code: "cu",
            name_de: "Kuba",
            name_en: "Cuba",
            calling_code: "+53",
        },
        CountryInfo {
            iso_code: "kw",
            name_de: "Kuwait",
            name_en: "Kuwait",
            calling_code: "+965",
        },
        // ═══ L ═══
        CountryInfo {
            iso_code: "la",
            name_de: "Laos",
            name_en: "Laos",
            calling_code: "+856",
        },
        CountryInfo {
            iso_code: "ls",
            name_de: "Lesotho",
            name_en: "Lesotho",
            calling_code: "+266",
        },
        CountryInfo {
            iso_code: "lv",
            name_de: "Lettland",
            name_en: "Latvia",
            calling_code: "+371",
        },
        CountryInfo {
            iso_code: "lb",
            name_de: "Libanon",
            name_en: "Lebanon",
            calling_code: "+961",
        },
        CountryInfo {
            iso_code: "lr",
            name_de: "Liberia",
            name_en: "Liberia",
            calling_code: "+231",
        },
        CountryInfo {
            iso_code: "ly",
            name_de: "Libyen",
            name_en: "Libya",
            calling_code: "+218",
        },
        CountryInfo {
            iso_code: "li",
            name_de: "Liechtenstein",
            name_en: "Liechtenstein",
            calling_code: "+423",
        },
        CountryInfo {
            iso_code: "lt",
            name_de: "Litauen",
            name_en: "Lithuania",
            calling_code: "+370",
        },
        CountryInfo {
            iso_code: "lu",
            name_de: "Luxemburg",
            name_en: "Luxembourg",
            calling_code: "+352",
        },
        // ═══ M ═══
        CountryInfo {
            iso_code: "mg",
            name_de: "Madagaskar",
            name_en: "Madagascar",
            calling_code: "+261",
        },
        CountryInfo {
            iso_code: "mw",
            name_de: "Malawi",
            name_en: "Malawi",
            calling_code: "+265",
        },
        CountryInfo {
            iso_code: "my",
            name_de: "Malaysia",
            name_en: "Malaysia",
            calling_code: "+60",
        },
        CountryInfo {
            iso_code: "mv",
            name_de: "Malediven",
            name_en: "Maldives",
            calling_code: "+960",
        },
        CountryInfo {
            iso_code: "ml",
            name_de: "Mali",
            name_en: "Mali",
            calling_code: "+223",
        },
        CountryInfo {
            iso_code: "mt",
            name_de: "Malta",
            name_en: "Malta",
            calling_code: "+356",
        },
        CountryInfo {
            iso_code: "ma",
            name_de: "Marokko",
            name_en: "Morocco",
            calling_code: "+212",
        },
        CountryInfo {
            iso_code: "mh",
            name_de: "Marshallinseln",
            name_en: "Marshall Islands",
            calling_code: "+692",
        },
        CountryInfo {
            iso_code: "mr",
            name_de: "Mauretanien",
            name_en: "Mauritania",
            calling_code: "+222",
        },
        CountryInfo {
            iso_code: "mu",
            name_de: "Mauritius",
            name_en: "Mauritius",
            calling_code: "+230",
        },
        CountryInfo {
            iso_code: "mx",
            name_de: "Mexiko",
            name_en: "Mexico",
            calling_code: "+52",
        },
        CountryInfo {
            iso_code: "fm",
            name_de: "Mikronesien",
            name_en: "Micronesia",
            calling_code: "+691",
        },
        CountryInfo {
            iso_code: "md",
            name_de: "Moldau",
            name_en: "Moldova",
            calling_code: "+373",
        },
        CountryInfo {
            iso_code: "mc",
            name_de: "Monaco",
            name_en: "Monaco",
            calling_code: "+377",
        },
        CountryInfo {
            iso_code: "mn",
            name_de: "Mongolei",
            name_en: "Mongolia",
            calling_code: "+976",
        },
        CountryInfo {
            iso_code: "me",
            name_de: "Montenegro",
            name_en: "Montenegro",
            calling_code: "+382",
        },
        CountryInfo {
            iso_code: "mz",
            name_de: "Mosambik",
            name_en: "Mozambique",
            calling_code: "+258",
        },
        CountryInfo {
            iso_code: "mm",
            name_de: "Myanmar",
            name_en: "Myanmar",
            calling_code: "+95",
        },
        // ═══ N ═══
        CountryInfo {
            iso_code: "na",
            name_de: "Namibia",
            name_en: "Namibia",
            calling_code: "+264",
        },
        CountryInfo {
            iso_code: "nr",
            name_de: "Nauru",
            name_en: "Nauru",
            calling_code: "+674",
        },
        CountryInfo {
            iso_code: "np",
            name_de: "Nepal",
            name_en: "Nepal",
            calling_code: "+977",
        },
        CountryInfo {
            iso_code: "nz",
            name_de: "Neuseeland",
            name_en: "New Zealand",
            calling_code: "+64",
        },
        CountryInfo {
            iso_code: "ni",
            name_de: "Nicaragua",
            name_en: "Nicaragua",
            calling_code: "+505",
        },
        CountryInfo {
            iso_code: "nl",
            name_de: "Niederlande",
            name_en: "Netherlands",
            calling_code: "+31",
        },
        CountryInfo {
            iso_code: "ne",
            name_de: "Niger",
            name_en: "Niger",
            calling_code: "+227",
        },
        CountryInfo {
            iso_code: "ng",
            name_de: "Nigeria",
            name_en: "Nigeria",
            calling_code: "+234",
        },
        CountryInfo {
            iso_code: "kp",
            name_de: "Nordkorea",
            name_en: "North Korea",
            calling_code: "+850",
        },
        CountryInfo {
            iso_code: "mk",
            name_de: "Nordmazedonien",
            name_en: "North Macedonia",
            calling_code: "+389",
        },
        CountryInfo {
            iso_code: "no",
            name_de: "Norwegen",
            name_en: "Norway",
            calling_code: "+47",
        },
        // ═══ O ═══
        CountryInfo {
            iso_code: "om",
            name_de: "Oman",
            name_en: "Oman",
            calling_code: "+968",
        },
        CountryInfo {
            iso_code: "at",
            name_de: "Österreich",
            name_en: "Austria",
            calling_code: "+43",
        },
        CountryInfo {
            iso_code: "tl",
            name_de: "Osttimor",
            name_en: "Timor-Leste",
            calling_code: "+670",
        },
        // ═══ P ═══
        CountryInfo {
            iso_code: "pk",
            name_de: "Pakistan",
            name_en: "Pakistan",
            calling_code: "+92",
        },
        CountryInfo {
            iso_code: "pw",
            name_de: "Palau",
            name_en: "Palau",
            calling_code: "+680",
        },
        CountryInfo {
            iso_code: "ps",
            name_de: "Palästina",
            name_en: "Palestine",
            calling_code: "+970",
        },
        CountryInfo {
            iso_code: "pa",
            name_de: "Panama",
            name_en: "Panama",
            calling_code: "+507",
        },
        CountryInfo {
            iso_code: "pg",
            name_de: "Papua-Neuguinea",
            name_en: "Papua New Guinea",
            calling_code: "+675",
        },
        CountryInfo {
            iso_code: "py",
            name_de: "Paraguay",
            name_en: "Paraguay",
            calling_code: "+595",
        },
        CountryInfo {
            iso_code: "pe",
            name_de: "Peru",
            name_en: "Peru",
            calling_code: "+51",
        },
        CountryInfo {
            iso_code: "ph",
            name_de: "Philippinen",
            name_en: "Philippines",
            calling_code: "+63",
        },
        CountryInfo {
            iso_code: "pl",
            name_de: "Polen",
            name_en: "Poland",
            calling_code: "+48",
        },
        CountryInfo {
            iso_code: "pt",
            name_de: "Portugal",
            name_en: "Portugal",
            calling_code: "+351",
        },
        // ═══ R ═══
        CountryInfo {
            iso_code: "rw",
            name_de: "Ruanda",
            name_en: "Rwanda",
            calling_code: "+250",
        },
        CountryInfo {
            iso_code: "ro",
            name_de: "Rumänien",
            name_en: "Romania",
            calling_code: "+40",
        },
        CountryInfo {
            iso_code: "ru",
            name_de: "Russland",
            name_en: "Russia",
            calling_code: "+7",
        },
        // ═══ S ═══
        CountryInfo {
            iso_code: "sb",
            name_de: "Salomonen",
            name_en: "Solomon Islands",
            calling_code: "+677",
        },
        CountryInfo {
            iso_code: "zm",
            name_de: "Sambia",
            name_en: "Zambia",
            calling_code: "+260",
        },
        CountryInfo {
            iso_code: "ws",
            name_de: "Samoa",
            name_en: "Samoa",
            calling_code: "+685",
        },
        CountryInfo {
            iso_code: "sm",
            name_de: "San Marino",
            name_en: "San Marino",
            calling_code: "+378",
        },
        CountryInfo {
            iso_code: "st",
            name_de: "São Tomé und Príncipe",
            name_en: "São Tomé and Príncipe",
            calling_code: "+239",
        },
        CountryInfo {
            iso_code: "sa",
            name_de: "Saudi-Arabien",
            name_en: "Saudi Arabia",
            calling_code: "+966",
        },
        CountryInfo {
            iso_code: "se",
            name_de: "Schweden",
            name_en: "Sweden",
            calling_code: "+46",
        },
        CountryInfo {
            iso_code: "ch",
            name_de: "Schweiz",
            name_en: "Switzerland",
            calling_code: "+41",
        },
        CountryInfo {
            iso_code: "sn",
            name_de: "Senegal",
            name_en: "Senegal",
            calling_code: "+221",
        },
        CountryInfo {
            iso_code: "rs",
            name_de: "Serbien",
            name_en: "Serbia",
            calling_code: "+381",
        },
        CountryInfo {
            iso_code: "sc",
            name_de: "Seychellen",
            name_en: "Seychelles",
            calling_code: "+248",
        },
        CountryInfo {
            iso_code: "sl",
            name_de: "Sierra Leone",
            name_en: "Sierra Leone",
            calling_code: "+232",
        },
        CountryInfo {
            iso_code: "zw",
            name_de: "Simbabwe",
            name_en: "Zimbabwe",
            calling_code: "+263",
        },
        CountryInfo {
            iso_code: "sg",
            name_de: "Singapur",
            name_en: "Singapore",
            calling_code: "+65",
        },
        CountryInfo {
            iso_code: "sk",
            name_de: "Slowakei",
            name_en: "Slovakia",
            calling_code: "+421",
        },
        CountryInfo {
            iso_code: "si",
            name_de: "Slowenien",
            name_en: "Slovenia",
            calling_code: "+386",
        },
        CountryInfo {
            iso_code: "so",
            name_de: "Somalia",
            name_en: "Somalia",
            calling_code: "+252",
        },
        CountryInfo {
            iso_code: "es",
            name_de: "Spanien",
            name_en: "Spain",
            calling_code: "+34",
        },
        CountryInfo {
            iso_code: "lk",
            name_de: "Sri Lanka",
            name_en: "Sri Lanka",
            calling_code: "+94",
        },
        CountryInfo {
            iso_code: "kn",
            name_de: "St. Kitts und Nevis",
            name_en: "Saint Kitts and Nevis",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "lc",
            name_de: "St. Lucia",
            name_en: "Saint Lucia",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "vc",
            name_de: "St. Vincent und die Grenadinen",
            name_en: "Saint Vincent and the Grenadines",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "sd",
            name_de: "Sudan",
            name_en: "Sudan",
            calling_code: "+249",
        },
        CountryInfo {
            iso_code: "za",
            name_de: "Südafrika",
            name_en: "South Africa",
            calling_code: "+27",
        },
        CountryInfo {
            iso_code: "kr",
            name_de: "Südkorea",
            name_en: "South Korea",
            calling_code: "+82",
        },
        CountryInfo {
            iso_code: "ss",
            name_de: "Südsudan",
            name_en: "South Sudan",
            calling_code: "+211",
        },
        CountryInfo {
            iso_code: "sr",
            name_de: "Suriname",
            name_en: "Suriname",
            calling_code: "+597",
        },
        CountryInfo {
            iso_code: "sy",
            name_de: "Syrien",
            name_en: "Syria",
            calling_code: "+963",
        },
        // ═══ T ═══
        CountryInfo {
            iso_code: "tj",
            name_de: "Tadschikistan",
            name_en: "Tajikistan",
            calling_code: "+992",
        },
        CountryInfo {
            iso_code: "tw",
            name_de: "Taiwan",
            name_en: "Taiwan",
            calling_code: "+886",
        },
        CountryInfo {
            iso_code: "tz",
            name_de: "Tansania",
            name_en: "Tanzania",
            calling_code: "+255",
        },
        CountryInfo {
            iso_code: "th",
            name_de: "Thailand",
            name_en: "Thailand",
            calling_code: "+66",
        },
        CountryInfo {
            iso_code: "tg",
            name_de: "Togo",
            name_en: "Togo",
            calling_code: "+228",
        },
        CountryInfo {
            iso_code: "to",
            name_de: "Tonga",
            name_en: "Tonga",
            calling_code: "+676",
        },
        CountryInfo {
            iso_code: "tt",
            name_de: "Trinidad und Tobago",
            name_en: "Trinidad and Tobago",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "td",
            name_de: "Tschad",
            name_en: "Chad",
            calling_code: "+235",
        },
        CountryInfo {
            iso_code: "cz",
            name_de: "Tschechien",
            name_en: "Czech Republic",
            calling_code: "+420",
        },
        CountryInfo {
            iso_code: "tn",
            name_de: "Tunesien",
            name_en: "Tunisia",
            calling_code: "+216",
        },
        CountryInfo {
            iso_code: "tr",
            name_de: "Türkei",
            name_en: "Turkey",
            calling_code: "+90",
        },
        CountryInfo {
            iso_code: "tm",
            name_de: "Turkmenistan",
            name_en: "Turkmenistan",
            calling_code: "+993",
        },
        CountryInfo {
            iso_code: "tv",
            name_de: "Tuvalu",
            name_en: "Tuvalu",
            calling_code: "+688",
        },
        // ═══ U ═══
        CountryInfo {
            iso_code: "ug",
            name_de: "Uganda",
            name_en: "Uganda",
            calling_code: "+256",
        },
        CountryInfo {
            iso_code: "ua",
            name_de: "Ukraine",
            name_en: "Ukraine",
            calling_code: "+380",
        },
        CountryInfo {
            iso_code: "hu",
            name_de: "Ungarn",
            name_en: "Hungary",
            calling_code: "+36",
        },
        CountryInfo {
            iso_code: "uy",
            name_de: "Uruguay",
            name_en: "Uruguay",
            calling_code: "+598",
        },
        CountryInfo {
            iso_code: "uz",
            name_de: "Usbekistan",
            name_en: "Uzbekistan",
            calling_code: "+998",
        },
        // ═══ V ═══
        CountryInfo {
            iso_code: "vu",
            name_de: "Vanuatu",
            name_en: "Vanuatu",
            calling_code: "+678",
        },
        CountryInfo {
            iso_code: "va",
            name_de: "Vatikanstadt",
            name_en: "Vatican City",
            calling_code: "+379",
        },
        CountryInfo {
            iso_code: "ve",
            name_de: "Venezuela",
            name_en: "Venezuela",
            calling_code: "+58",
        },
        CountryInfo {
            iso_code: "ae",
            name_de: "Vereinigte Arabische Emirate",
            name_en: "United Arab Emirates",
            calling_code: "+971",
        },
        CountryInfo {
            iso_code: "us",
            name_de: "Vereinigte Staaten",
            name_en: "United States",
            calling_code: "+1",
        },
        CountryInfo {
            iso_code: "gb",
            name_de: "Vereinigtes Königreich",
            name_en: "United Kingdom",
            calling_code: "+44",
        },
        CountryInfo {
            iso_code: "vn",
            name_de: "Vietnam",
            name_en: "Vietnam",
            calling_code: "+84",
        },
        // ═══ Z ═══
        CountryInfo {
            iso_code: "cf",
            name_de: "Zentralafrikanische Republik",
            name_en: "Central African Republic",
            calling_code: "+236",
        },
        CountryInfo {
            iso_code: "cy",
            name_de: "Zypern",
            name_en: "Cyprus",
            calling_code: "+357",
        },
    ]
}

/// Returns the index of Germany ("de") in the country list.
///
/// Germany is the default selection for the phone prefix dropdown.
pub fn default_country_index() -> u32 {
    countries()
        .iter()
        .position(|c| c.iso_code == "de")
        .expect("Germany must be present in the country list") as u32
}

/// Builds a display string for a country entry, e.g., "DE +49".
///
/// The ISO code is shown in uppercase followed by the calling code.
#[allow(dead_code)]
pub fn country_display_line(info: &CountryInfo) -> String {
    format!("{} {}", info.iso_code.to_uppercase(), info.calling_code)
}

/// Returns the index in the country list for the given calling code (e.g. "+49").
///
/// Falls back to Germany if the code is not found.
pub fn country_index_by_code(code: &str) -> u32 {
    countries()
        .iter()
        .position(|c| c.calling_code == code)
        .unwrap_or_else(|| default_country_index() as usize) as u32
}

/// Returns the filesystem path to the flag SVG for the given ISO country code.
///
/// In development mode, the path is resolved relative to the project root
/// (`flags/{iso_code}.svg`). In production, it falls back to a path relative
/// to the current executable's directory.
pub fn flag_svg_path(iso_code: &str) -> PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    let dev_path = PathBuf::from(format!("flags/{}.svg", iso_code));
    if dev_path.exists() {
        return dev_path;
    }

    exe_dir.join("flags").join(format!("{}.svg", iso_code))
}
