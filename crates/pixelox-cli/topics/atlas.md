# Bundled map data and JavaScript libraries

`render d3` and `render three` load bundled libraries, so a script needs no `--lib`. Every script (`render js` too) can read the bundled maps with `px.atlas(name)`, which returns TopoJSON; `topojson.feature()` turns an object of it into GeoJSON for `d3.geoPath()`. This page is generated from the bundled files (ADR 0018); the cookbook `docs/guides/cookbook/charts-and-maps-with-d3.md` has complete charts and maps.

## Libraries (`--with`)

| `--with` | Global | Package | Version | License | What it gives |
| --- | --- | --- | --- | --- | --- |
| `d3` | `d3` | d3 | 7.9.0 | ISC | the whole d3 (scales, axes, shapes, layouts, hierarchy, geo projections, formats) |
| `three` | `THREE` | three | 0.187.0 | MIT | three.js core plus THREE.SVGRenderer, THREE.Projector, and THREE.RoundedBoxGeometry |
| `topojson` | `topojson` | topojson-client | 3.1.0 | ISC | topojson-client: feature, mesh, merge, neighbors for px.atlas() data |

Defaults: `render d3` loads `d3,topojson`, `render three` loads `three,topojson`, `render js` loads nothing (`--with d3` opts in). A `--lib` file replaces the bundled `d3`/`three` unless `--with` names it; with `--with d3 --lib d3-sankey.js` a plugin extends the bundled d3. The versions that ran are recorded in the sidecar (`bundled`) and pinned in the recipe (`with: d3@7.9.0`).

## Atlases (`px.atlas(name)`)

| Name | Objects | Source | What it is |
| --- | --- | --- | --- |
| `world-110m` | `countries`, `land` | world-atlas 2.0.2 `countries-110m.json` | 177 countries and the land, Natural Earth 1:110m; longitude/latitude (any d3 projection) |
| `world-50m` | `countries`, `land` | world-atlas 2.0.2 `countries-50m.json` | 241 countries and territories and the land, Natural Earth 1:50m (finer coasts, small islands) |
| `land-110m` | `land` | world-atlas 2.0.2 `land-110m.json` | the land masses only, Natural Earth 1:110m (a light background or a globe) |
| `us-states` | `states`, `nation` | us-atlas 3.0.1 `states-10m.json` | 50 states, DC, and 5 territories, Census 2017; longitude/latitude: draw with d3.geoAlbersUsa() (Alaska and Hawaii as insets) |
| `us-states-albers` | `states`, `nation` | us-atlas 3.0.1 `states-albers-10m.json` | 50 states and DC already projected (Albers USA, insets) into a 975x610 frame: draw with d3.geoPath() and no projection |
| `us-nation` | `nation` | us-atlas 3.0.1 `nation-10m.json` | the U.S. outline, Census 2017; longitude/latitude |

Geometry: Natural Earth (public domain) through world-atlas (ISC); U.S. Census Bureau cartographic boundaries (public domain) through us-atlas (ISC). Boundaries are Natural Earth's de facto lines.

```js
const us = px.atlas('us-states');                       // TopoJSON
const states = topojson.feature(us, us.objects.states);   // GeoJSON FeatureCollection
const borders = topojson.mesh(us, us.objects.states, (a, b) => a !== b);
const path = d3.geoPath(d3.geoAlbersUsa().fitSize([960, 600], states));
// each feature: id '06', properties {name: 'California', postal: 'CA', fips: '06'}
const world = px.atlas('world-110m');
const countries = topojson.feature(world, world.objects.countries);
// each country: id '840', properties {name: 'United States of America', iso_a2: 'US', iso_a3: 'USA'}
```

`px.atlases()` returns this list inside a script.

## U.S. states (`us-states`, `us-states-albers`)

`id` is the FIPS code (a string). Join data on `properties.postal` (`CA`), `properties.name`, or `id`. The five territories (AS, GU, MP, PR, VI) are only in `us-states`, and `d3.geoAlbersUsa()` draws none of them (it projects the 50 states and DC only).

| id (FIPS) | postal | name | in us-states-albers |
| --- | --- | --- | --- |
| 01 | AL | Alabama | yes |
| 02 | AK | Alaska | yes |
| 04 | AZ | Arizona | yes |
| 05 | AR | Arkansas | yes |
| 06 | CA | California | yes |
| 08 | CO | Colorado | yes |
| 09 | CT | Connecticut | yes |
| 10 | DE | Delaware | yes |
| 11 | DC | District of Columbia | yes |
| 12 | FL | Florida | yes |
| 13 | GA | Georgia | yes |
| 15 | HI | Hawaii | yes |
| 16 | ID | Idaho | yes |
| 17 | IL | Illinois | yes |
| 18 | IN | Indiana | yes |
| 19 | IA | Iowa | yes |
| 20 | KS | Kansas | yes |
| 21 | KY | Kentucky | yes |
| 22 | LA | Louisiana | yes |
| 23 | ME | Maine | yes |
| 24 | MD | Maryland | yes |
| 25 | MA | Massachusetts | yes |
| 26 | MI | Michigan | yes |
| 27 | MN | Minnesota | yes |
| 28 | MS | Mississippi | yes |
| 29 | MO | Missouri | yes |
| 30 | MT | Montana | yes |
| 31 | NE | Nebraska | yes |
| 32 | NV | Nevada | yes |
| 33 | NH | New Hampshire | yes |
| 34 | NJ | New Jersey | yes |
| 35 | NM | New Mexico | yes |
| 36 | NY | New York | yes |
| 37 | NC | North Carolina | yes |
| 38 | ND | North Dakota | yes |
| 39 | OH | Ohio | yes |
| 40 | OK | Oklahoma | yes |
| 41 | OR | Oregon | yes |
| 42 | PA | Pennsylvania | yes |
| 44 | RI | Rhode Island | yes |
| 45 | SC | South Carolina | yes |
| 46 | SD | South Dakota | yes |
| 47 | TN | Tennessee | yes |
| 48 | TX | Texas | yes |
| 49 | UT | Utah | yes |
| 50 | VT | Vermont | yes |
| 51 | VA | Virginia | yes |
| 53 | WA | Washington | yes |
| 54 | WV | West Virginia | yes |
| 55 | WI | Wisconsin | yes |
| 56 | WY | Wyoming | yes |
| 60 | AS | American Samoa | no |
| 66 | GU | Guam | no |
| 69 | MP | Commonwealth of the Northern Mariana Islands | no |
| 72 | PR | Puerto Rico | no |
| 78 | VI | United States Virgin Islands | no |

## Countries (`world-110m`, `world-50m`)

`id` is the ISO 3166-1 numeric code (a string, `"036"`); `iso_a2` and `iso_a3` are added by pixelox (Kosovo gets the user-assigned XK / XKX). Names are Natural Earth's short names (`Dem. Rep. Congo`, `United States of America`): join data on `iso_a3` where you can. A few features have no code (N. Cyprus, Somaliland, Siachen Glacier, Indian Ocean Ter.). In `world-50m`, Ashmore and Cartier Is. share Australia's id.

| id | iso_a2 | iso_a3 | name | in world-110m |
| --- | --- | --- | --- | --- |
| 004 | AF | AFG | Afghanistan | yes |
| 008 | AL | ALB | Albania | yes |
| 012 | DZ | DZA | Algeria | yes |
| 016 | AS | ASM | American Samoa | no |
| 020 | AD | AND | Andorra | no |
| 024 | AO | AGO | Angola | yes |
| 660 | AI | AIA | Anguilla | no |
| 010 | AQ | ATA | Antarctica | yes |
| 028 | AG | ATG | Antigua and Barb. | no |
| 032 | AR | ARG | Argentina | yes |
| 051 | AM | ARM | Armenia | yes |
| 533 | AW | ABW | Aruba | no |
| 036 | AU | AUS | Ashmore and Cartier Is. | no |
| 036 | AU | AUS | Australia | yes |
| 040 | AT | AUT | Austria | yes |
| 031 | AZ | AZE | Azerbaijan | yes |
| 044 | BS | BHS | Bahamas | yes |
| 048 | BH | BHR | Bahrain | no |
| 050 | BD | BGD | Bangladesh | yes |
| 052 | BB | BRB | Barbados | no |
| 112 | BY | BLR | Belarus | yes |
| 056 | BE | BEL | Belgium | yes |
| 084 | BZ | BLZ | Belize | yes |
| 204 | BJ | BEN | Benin | yes |
| 060 | BM | BMU | Bermuda | no |
| 064 | BT | BTN | Bhutan | yes |
| 068 | BO | BOL | Bolivia | yes |
| 070 | BA | BIH | Bosnia and Herz. | yes |
| 072 | BW | BWA | Botswana | yes |
| 086 | IO | IOT | Br. Indian Ocean Ter. | no |
| 076 | BR | BRA | Brazil | yes |
| 092 | VG | VGB | British Virgin Is. | no |
| 096 | BN | BRN | Brunei | yes |
| 100 | BG | BGR | Bulgaria | yes |
| 854 | BF | BFA | Burkina Faso | yes |
| 108 | BI | BDI | Burundi | yes |
| 132 | CV | CPV | Cabo Verde | no |
| 116 | KH | KHM | Cambodia | yes |
| 120 | CM | CMR | Cameroon | yes |
| 124 | CA | CAN | Canada | yes |
| 136 | KY | CYM | Cayman Is. | no |
| 140 | CF | CAF | Central African Rep. | yes |
| 148 | TD | TCD | Chad | yes |
| 152 | CL | CHL | Chile | yes |
| 156 | CN | CHN | China | yes |
| 170 | CO | COL | Colombia | yes |
| 174 | KM | COM | Comoros | no |
| 178 | CG | COG | Congo | yes |
| 184 | CK | COK | Cook Is. | no |
| 188 | CR | CRI | Costa Rica | yes |
| 191 | HR | HRV | Croatia | yes |
| 192 | CU | CUB | Cuba | yes |
| 531 | CW | CUW | Curaçao | no |
| 196 | CY | CYP | Cyprus | yes |
| 203 | CZ | CZE | Czechia | yes |
| 384 | CI | CIV | Côte d'Ivoire | yes |
| 180 | CD | COD | Dem. Rep. Congo | yes |
| 208 | DK | DNK | Denmark | yes |
| 262 | DJ | DJI | Djibouti | yes |
| 212 | DM | DMA | Dominica | no |
| 214 | DO | DOM | Dominican Rep. | yes |
| 218 | EC | ECU | Ecuador | yes |
| 818 | EG | EGY | Egypt | yes |
| 222 | SV | SLV | El Salvador | yes |
| 226 | GQ | GNQ | Eq. Guinea | yes |
| 232 | ER | ERI | Eritrea | yes |
| 233 | EE | EST | Estonia | yes |
| 231 | ET | ETH | Ethiopia | yes |
| 234 | FO | FRO | Faeroe Is. | no |
| 238 | FK | FLK | Falkland Is. | yes |
| 242 | FJ | FJI | Fiji | yes |
| 246 | FI | FIN | Finland | yes |
| 258 | PF | PYF | Fr. Polynesia | no |
| 260 | TF | ATF | Fr. S. Antarctic Lands | yes |
| 250 | FR | FRA | France | yes |
| 266 | GA | GAB | Gabon | yes |
| 270 | GM | GMB | Gambia | yes |
| 268 | GE | GEO | Georgia | yes |
| 276 | DE | DEU | Germany | yes |
| 288 | GH | GHA | Ghana | yes |
| 300 | GR | GRC | Greece | yes |
| 304 | GL | GRL | Greenland | yes |
| 308 | GD | GRD | Grenada | no |
| 316 | GU | GUM | Guam | no |
| 320 | GT | GTM | Guatemala | yes |
| 831 | GG | GGY | Guernsey | no |
| 324 | GN | GIN | Guinea | yes |
| 624 | GW | GNB | Guinea-Bissau | yes |
| 328 | GY | GUY | Guyana | yes |
| 332 | HT | HTI | Haiti | yes |
| 334 | HM | HMD | Heard I. and McDonald Is. | no |
| 340 | HN | HND | Honduras | yes |
| 344 | HK | HKG | Hong Kong | no |
| 348 | HU | HUN | Hungary | yes |
| 352 | IS | ISL | Iceland | yes |
| 356 | IN | IND | India | yes |
| - | - | - | Indian Ocean Ter. | no |
| 360 | ID | IDN | Indonesia | yes |
| 364 | IR | IRN | Iran | yes |
| 368 | IQ | IRQ | Iraq | yes |
| 372 | IE | IRL | Ireland | yes |
| 833 | IM | IMN | Isle of Man | no |
| 376 | IL | ISR | Israel | yes |
| 380 | IT | ITA | Italy | yes |
| 388 | JM | JAM | Jamaica | yes |
| 392 | JP | JPN | Japan | yes |
| 832 | JE | JEY | Jersey | no |
| 400 | JO | JOR | Jordan | yes |
| 398 | KZ | KAZ | Kazakhstan | yes |
| 404 | KE | KEN | Kenya | yes |
| 296 | KI | KIR | Kiribati | no |
| - | XK | XKX | Kosovo | yes |
| 414 | KW | KWT | Kuwait | yes |
| 417 | KG | KGZ | Kyrgyzstan | yes |
| 418 | LA | LAO | Laos | yes |
| 428 | LV | LVA | Latvia | yes |
| 422 | LB | LBN | Lebanon | yes |
| 426 | LS | LSO | Lesotho | yes |
| 430 | LR | LBR | Liberia | yes |
| 434 | LY | LBY | Libya | yes |
| 438 | LI | LIE | Liechtenstein | no |
| 440 | LT | LTU | Lithuania | yes |
| 442 | LU | LUX | Luxembourg | yes |
| 446 | MO | MAC | Macao | no |
| 807 | MK | MKD | Macedonia | yes |
| 450 | MG | MDG | Madagascar | yes |
| 454 | MW | MWI | Malawi | yes |
| 458 | MY | MYS | Malaysia | yes |
| 462 | MV | MDV | Maldives | no |
| 466 | ML | MLI | Mali | yes |
| 470 | MT | MLT | Malta | no |
| 584 | MH | MHL | Marshall Is. | no |
| 478 | MR | MRT | Mauritania | yes |
| 480 | MU | MUS | Mauritius | no |
| 484 | MX | MEX | Mexico | yes |
| 583 | FM | FSM | Micronesia | no |
| 498 | MD | MDA | Moldova | yes |
| 492 | MC | MCO | Monaco | no |
| 496 | MN | MNG | Mongolia | yes |
| 499 | ME | MNE | Montenegro | yes |
| 500 | MS | MSR | Montserrat | no |
| 504 | MA | MAR | Morocco | yes |
| 508 | MZ | MOZ | Mozambique | yes |
| 104 | MM | MMR | Myanmar | yes |
| - | - | - | N. Cyprus | yes |
| 580 | MP | MNP | N. Mariana Is. | no |
| 516 | NA | NAM | Namibia | yes |
| 520 | NR | NRU | Nauru | no |
| 524 | NP | NPL | Nepal | yes |
| 528 | NL | NLD | Netherlands | yes |
| 540 | NC | NCL | New Caledonia | yes |
| 554 | NZ | NZL | New Zealand | yes |
| 558 | NI | NIC | Nicaragua | yes |
| 562 | NE | NER | Niger | yes |
| 566 | NG | NGA | Nigeria | yes |
| 570 | NU | NIU | Niue | no |
| 574 | NF | NFK | Norfolk Island | no |
| 408 | KP | PRK | North Korea | yes |
| 578 | NO | NOR | Norway | yes |
| 512 | OM | OMN | Oman | yes |
| 586 | PK | PAK | Pakistan | yes |
| 585 | PW | PLW | Palau | no |
| 275 | PS | PSE | Palestine | yes |
| 591 | PA | PAN | Panama | yes |
| 598 | PG | PNG | Papua New Guinea | yes |
| 600 | PY | PRY | Paraguay | yes |
| 604 | PE | PER | Peru | yes |
| 608 | PH | PHL | Philippines | yes |
| 612 | PN | PCN | Pitcairn Is. | no |
| 616 | PL | POL | Poland | yes |
| 620 | PT | PRT | Portugal | yes |
| 630 | PR | PRI | Puerto Rico | yes |
| 634 | QA | QAT | Qatar | yes |
| 642 | RO | ROU | Romania | yes |
| 643 | RU | RUS | Russia | yes |
| 646 | RW | RWA | Rwanda | yes |
| 239 | GS | SGS | S. Geo. and the Is. | no |
| 728 | SS | SSD | S. Sudan | yes |
| 654 | SH | SHN | Saint Helena | no |
| 662 | LC | LCA | Saint Lucia | no |
| 882 | WS | WSM | Samoa | no |
| 674 | SM | SMR | San Marino | no |
| 682 | SA | SAU | Saudi Arabia | yes |
| 686 | SN | SEN | Senegal | yes |
| 688 | RS | SRB | Serbia | yes |
| 690 | SC | SYC | Seychelles | no |
| - | - | - | Siachen Glacier | no |
| 694 | SL | SLE | Sierra Leone | yes |
| 702 | SG | SGP | Singapore | no |
| 534 | SX | SXM | Sint Maarten | no |
| 703 | SK | SVK | Slovakia | yes |
| 705 | SI | SVN | Slovenia | yes |
| 090 | SB | SLB | Solomon Is. | yes |
| 706 | SO | SOM | Somalia | yes |
| - | - | - | Somaliland | yes |
| 710 | ZA | ZAF | South Africa | yes |
| 410 | KR | KOR | South Korea | yes |
| 724 | ES | ESP | Spain | yes |
| 144 | LK | LKA | Sri Lanka | yes |
| 652 | BL | BLM | St-Barthélemy | no |
| 663 | MF | MAF | St-Martin | no |
| 659 | KN | KNA | St. Kitts and Nevis | no |
| 666 | PM | SPM | St. Pierre and Miquelon | no |
| 670 | VC | VCT | St. Vin. and Gren. | no |
| 729 | SD | SDN | Sudan | yes |
| 740 | SR | SUR | Suriname | yes |
| 752 | SE | SWE | Sweden | yes |
| 756 | CH | CHE | Switzerland | yes |
| 760 | SY | SYR | Syria | yes |
| 678 | ST | STP | São Tomé and Principe | no |
| 158 | TW | TWN | Taiwan | yes |
| 762 | TJ | TJK | Tajikistan | yes |
| 834 | TZ | TZA | Tanzania | yes |
| 764 | TH | THA | Thailand | yes |
| 626 | TL | TLS | Timor-Leste | yes |
| 768 | TG | TGO | Togo | yes |
| 776 | TO | TON | Tonga | no |
| 780 | TT | TTO | Trinidad and Tobago | yes |
| 788 | TN | TUN | Tunisia | yes |
| 792 | TR | TUR | Turkey | yes |
| 795 | TM | TKM | Turkmenistan | yes |
| 796 | TC | TCA | Turks and Caicos Is. | no |
| 850 | VI | VIR | U.S. Virgin Is. | no |
| 800 | UG | UGA | Uganda | yes |
| 804 | UA | UKR | Ukraine | yes |
| 784 | AE | ARE | United Arab Emirates | yes |
| 826 | GB | GBR | United Kingdom | yes |
| 840 | US | USA | United States of America | yes |
| 858 | UY | URY | Uruguay | yes |
| 860 | UZ | UZB | Uzbekistan | yes |
| 548 | VU | VUT | Vanuatu | yes |
| 336 | VA | VAT | Vatican | no |
| 862 | VE | VEN | Venezuela | yes |
| 704 | VN | VNM | Vietnam | yes |
| 732 | EH | ESH | W. Sahara | yes |
| 876 | WF | WLF | Wallis and Futuna Is. | no |
| 887 | YE | YEM | Yemen | yes |
| 894 | ZM | ZMB | Zambia | yes |
| 716 | ZW | ZWE | Zimbabwe | yes |
| 748 | SZ | SWZ | eSwatini | yes |
| 248 | AX | ALA | Åland | no |
