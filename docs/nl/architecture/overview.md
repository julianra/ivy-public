<!-- doc-key: architecture-overview -->
<!-- parity-version: 2 -->
<!-- sections: rules,components,trust,compatibility,gaps,visuals -->

# Architectuur en compatibiliteit

**Nederlands** | [English](../../en/architecture/overview.md)

<!-- section: rules -->
## Stabiele publieke regels

Core is een platformautoriteit, geen domeinbrein. Capabilitydomeinen hebben één
exclusieve geïnstalleerde eigenaar; overlappende voorouder- en
afstammelingdomeinen zijn verboden. Installatie, ownership, provider offer en
consumertoestemming zijn afzonderlijke beslissingen. Events zijn feiten, widgets
zijn presentatie en geen van beide is een verborgen capabilitygrant. Extern
verkeer eindigt bij Bridge en niet in Core.

<!-- section: components -->
## Componenten

| Component | Verantwoordelijkheid | Publieke broncode nu |
|---|---|---|
| Core | Register, grants, routing, lifecycle, audit, package trust | Nee; proprietary |
| Core UI | Lokaal platformbeheer | Nee; proprietary |
| Prism | Generieke UI en declaratieve widgets | Nee; alleen gedocumenteerd |
| Bridge | Externe netwerk- en kanaalgrens | Nee; alleen gedocumenteerd |
| Vault | Duurzame lokale storage-capabilityfamilie | Nee; contracten beschreven |
| Flow | Duurzame routines, retries en approvals | Nee; private implementatie |
| Agenda/Pulse/BookMe | Domeincapabilities | Nee; alleen status en toepassingen |
| Developer Kit | Publieke contracten, simulator, tools en voorbeelden | Ja; experimenteel |

<!-- section: trust -->
## Vertrouwen en invocation

Production Core controleert geauthenticeerde calleridentiteit, gedeclareerde
behoefte, consumergrant, exclusief providerownership, provider offer grant,
schemacompatibiliteit, health, trusted context en audit. De publieke simulator
behandelt bewust alleen manifestvalidatie, statische declaraties, ownership,
beschikbaarheid, exacte offer grants, exacte consumergrants, routing en
revocation.

Module-ID's in de simulator zijn testidentiteiten. Ze vervangen geen
geauthenticeerde productieprincipals. Dynamic capability requests staan in het
schema, maar worden in deze preview niet goedgekeurd of gesimuleerd.

<!-- section: compatibility -->
## Compatibiliteitsmatrix

| Onderwerp | Publieke documentatie | Simulator | Publieke contractsubset | Private Core | Ontbrekende integratie |
|---|---|---|---|---|---|
| Domeinownership | Normatieve samenvatting | Segmentoverlap afgedwongen | `capability_domains.owns` | Volledige installatieautoriteit | Package-naar-installatiebrug |
| Provideroffers | Uitgelegd | Exacte grant/revoke | Provided-capabilitydeclaratie | Geauthenticeerd offerbeleid | Publieke admin/runtime-API |
| Consumertoegang | Uitgelegd | Statische declaratie plus exacte grant | Required-capabilitydeclaratie | Geauthenticeerde caller en grants | Productie-identiteitsbinding |
| Dynamic requirements | Alleen architectuur | Niet ondersteund | Flag behouden | Goedgekeurde runtime requests | Publiek goedkeuringsprotocol |
| Lifecycle/health | Uitgelegd | Alleen beschikbaar/onbeschikbaar/verwijderd | Niet gecodeerd | Duurzame lifecycle en health | Publieke runtimeadapter |
| Audit/persistentie | Uitgelegd | Niet ondersteund | Niet gecodeerd | Duurzaam en begrensd | Publiek testprotocol |
| Signing | Trustmodel | Development-purpose signatures | Geen officiële packageclaim | Beschermde officiële signing | Publieke packagespecificatie |
| Widgets/events | Concepten | Alleen fixturepreview | Alleen developerproto | Registratie/routing/delivery | Stabiele publieke wiregeneratie |

<!-- section: gaps -->
## Wat een externe productiemodule nog nodig heeft

Voor een derde een echte productiemodule kan bouwen, verpakken, ondertekenen en
installeren, heeft Ivy nog een stabiel publiek runtimeprotocol, gegenereerde
bindings, package-layoutspecificatie, officiële signing/provenanceroute,
installatie-API, sandbox/resourcecontract, compatibiliteitsbeleid en
Marketplace-reviewrecord nodig. Deze repository verbergt die ontbrekende stappen
niet met een schijnruntime.

<!-- section: visuals -->
## Visuele status

De diagrammen in deze documentatie beschrijven gereviewde architectuur. Er zijn
geen productscreenshots opgenomen: bestaande Prism- en Core UI-schermen vereisen
nog een privacyreview. Geen mockinterface wordt als geïmplementeerde functionaliteit
voorgesteld.
