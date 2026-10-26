<!-- doc-key: architecture-overview -->
<!-- parity-version: 2 -->
<!-- sections: rules,components,trust,compatibility,gaps,visuals -->

# Architectuur en compatibiliteit

**Nederlands** | [English](../../en/architecture/overview.md)

## Wat is Ivy en wat leer je op deze pagina?

Ivy is een modulair softwareplatform: verschillende modules leveren elk hun eigen functies, maar kunnen via gezamenlijke afspraken samenwerken. Zo kan een module afspraken beheren, een andere gegevens opslaan en weer een andere automatiseringen uitvoeren. Je hoeft niet één groot programma te bouwen dat alles zelf doet.

**Ivy Core is de centrale beheerder, niet de uitvoerder van alle taken.** Core houdt bij welke modules er zijn, welke functies ze aanbieden en welke andere modules toestemming hebben om die functies te gebruiken. De gespecialiseerde kennis blijft bij de modules.

Deze pagina legt uit hoe de onderdelen samenwerken, waarom Ivy rechten strikt scheidt en wat een externe ontwikkelaar **nu al kan testen** tegenover wat nog niet als publieke productie-interface beschikbaar is.

> **Status:** deze repository bevat een experimentele Developer Kit, geen volledige publieke Ivy-runtime. Gedocumenteerd architectuurgedrag is niet automatisch beschikbaar als publieke implementatie.

### Een voorbeeld: slimme verlichting

Stel dat een ontwikkelaar een module bouwt die lampen aanstuurt:

1. De module **beschrijft** zichzelf en haar functies in een manifest.
2. Ivy controleert bij installatie en registratie welke module het verlichtingsdomein mag beheren.
3. Een automatiseringsmodule kan verklaren dat ze de functie ‘lamp inschakelen’ nodig heeft.
4. Dat verzoek mag pas door wanneer de afzonderlijke voorwaarden en toestemmingen zijn vervuld.
5. Prism kan een knop tonen en een event kan melden dat de lamp aanstaat, maar een knop of melding **geeft zelf geen toestemming**.

Dit is een illustratie van het ontwerp, geen claim dat dit specifieke voorbeeld al als volledig publiek productiescenario beschikbaar is.

<!-- section: rules -->
## De belangrijkste publieke afspraken

- **Core beheert het platform; modules beheren hun eigen vakgebied.** Core beslist over registratie, toegang en routering, niet over hoe een lamp, agenda of boeking inhoudelijk werkt.
- **Eén geïnstalleerde eigenaar per capabilitydomein.** Een capability is een benoemde functie die een module aanbiedt. Domeinen groeperen die functies. Als een module `home.lighting` bezit, kan een andere module niet ook `home.lighting.kitchen` bezitten: ouder- en kinddomeinen mogen niet overlappen.
- **Installatie is niet hetzelfde als toestemming.** Een module installeren, een domein toewijzen, een functie mogen aanbieden (*provider offer*) en die functie mogen gebruiken (*consumergrant*) zijn vier afzonderlijke beslissingen.
- **Een event is informatie, geen opdracht of recht.** Bijvoorbeeld: ‘de lamp is ingeschakeld’.
- **Een widget is een gebruikersinterface, geen machtiging.** Een knop in Prism mag geen beveiligingscontrole omzeilen.
- **Bridge is de grens met de buitenwereld.** Extern netwerkverkeer eindigt bij Bridge en gaat niet rechtstreeks naar Core.

<!-- section: components -->
## Welke onderdelen heeft Ivy?

| Onderdeel | Wat doet het in gewone taal? | Publieke broncode in deze repository? |
|---|---|---|
| Core | Centrale registratie, toegangsrechten, routering, levenscyclus, audit en vertrouwen in pakketten | Nee; proprietair |
| Core UI | Lokale schermen om het platform te beheren | Nee; proprietair |
| Prism | Algemene gebruikersinterface met declaratieve widgets | Nee; alleen gedocumenteerd |
| Bridge | Bewaakte grens voor externe netwerken en communicatiekanalen | Nee; alleen gedocumenteerd |
| Vault | Duurzame lokale opslag die modules via capabilities kunnen gebruiken | Nee; contracten beschreven |
| Flow | Routines en automatiseringen, waaronder opnieuw proberen en goedkeuringen | Nee; private implementatie |
| Agenda / Pulse / BookMe | Gespecialiseerde domeinfunctionaliteit, zoals afspraken en boekingen | Nee; alleen status en toepassingen beschreven |
| Developer Kit | Publieke contracten, simulator, hulpmiddelen en voorbeelden voor ontwikkelaars | Ja; experimenteel |

De Developer Kit is dus een **oefen- en controleomgeving voor ontwikkelaars**, niet een publieke kopie van Core.

<!-- section: trust -->
## Hoe beslist Ivy of een module iets mag uitvoeren?

Stel: module A wil een functie van module B aanroepen. In productie controleert Core onder andere:

1. **Wie vraagt het?** De identiteit van de aanroeper moet geauthenticeerd zijn.
2. **Was die functie gedeclareerd als behoefte?** De consumer moet haar afhankelijkheid correct hebben beschreven.
3. **Mag de aanroeper deze functie gebruiken?** Daarvoor is een consumergrant nodig.
4. **Is de provider de exclusieve eigenaar van het juiste domein?**
5. **Mag de provider deze exacte functie aanbieden?** Daarvoor geldt afzonderlijk een provider-offergrant.
6. **Zijn de gegevensschema's compatibel, is de provider beschikbaar en voldoet de aanvraag aan de vertrouwenscontext?**
7. **Wordt de relevante activiteit geaudit?**

Een *grant* is een expliciete toestemming. Een *provider* biedt een functie aan; een *consumer* gebruikt die functie. Een *invocation* is een concrete functieaanroep.

### Wat test de publieke simulator wel?

De simulator behandelt bewust alleen manifestvalidatie, statische declaraties, domeineigendom, beschikbaarheid, exacte provider-offergrants, exacte consumergrants, routering en het intrekken van rechten (*revocation*).

Hij gebruikt module-ID's als **testidentiteiten**: dat zijn geen geauthenticeerde productie-identiteiten. Dynamische capabilityverzoeken staan wel in het schema, maar worden in deze preview niet goedgekeurd of gesimuleerd. De simulator bewijst dus niet dat het volledige productiebeveiligingsmodel is getest.

<!-- section: compatibility -->
## Wat is al publiek beschikbaar en wat nog niet?

De tabel maakt onderscheid tussen documentatie, een beperkte testomgeving, publieke contracten en de private productie-implementatie.

| Onderwerp | Publieke documentatie | Simulator | Publieke contractsubset | Private Core | Wat ontbreekt publiek nog? |
|---|---|---|---|---|---|
| Domeineigendom | Regels uitgelegd | Controle op overlappende naamsegmenten | `capability_domains.owns` | Volledige installatieautoriteit | Koppeling van pakket naar installatie |
| Functies aanbieden | Regels uitgelegd | Exacte grants toekennen/intrekken | Declaratie van aangeboden capability | Geauthenticeerd offerbeleid | Publieke beheer-/runtime-API |
| Functies gebruiken | Regels uitgelegd | Statische declaratie plus exacte grant | Declaratie van benodigde capability | Geauthenticeerde aanroeper en grants | Koppeling met productie-identiteit |
| Dynamische behoeften | Architectuur beschreven | Niet ondersteund | Flag aanwezig | Goedgekeurde runtimeverzoeken | Publiek goedkeuringsprotocol |
| Levenscyclus en gezondheid | Gedrag beschreven | Alleen beschikbaar/onbeschikbaar/verwijderd | Niet vastgelegd | Duurzame lifecycle en health | Publieke runtimeadapter |
| Audit en persistentie | Gedrag beschreven | Niet ondersteund | Niet vastgelegd | Duurzaam en begrensd | Publiek testprotocol |
| Ondertekening | Vertrouwensmodel beschreven | Handtekeningen voor ontwikkeltests | Geen officiële pakketclaim | Beschermde officiële ondertekening | Publieke pakketspecificatie |
| Widgets en events | Concepten beschreven | Alleen preview met testvoorbeelden | Alleen developer-protobuf | Registratie, routering en aflevering | Stabiele publieke wiregeneratie |

**Hoe lees je dit?** ‘Beschreven’ betekent niet ‘publiek uitvoerbaar’. ‘Werkt in de simulator’ betekent niet ‘werkt al via een publieke productie-API’. En ‘niet publiek’ betekent niet noodzakelijk ‘bestaat niet intern’.

<!-- section: gaps -->
## Wat moet nog beschikbaar komen voor echte externe productiemodules?

Een externe ontwikkelaar heeft uiteindelijk meer nodig dan een geldig manifest en geslaagde simulatortests:

- Een **stabiel runtimeprotocol** en gegenereerde **taalbindings** om met Ivy te communiceren.
- Een **pakketspecificatie** met de indeling van de bestanden.
- Een officiële route voor **ondertekening en herkomstcontrole** van pakketten.
- Een publieke **installatie-API**.
- Een **sandbox- en resourcecontract**: welke systeembronnen een module mag gebruiken.
- Een **compatibiliteitsbeleid**: welke updates bestaande modules blijven ondersteunen.
- Een **Marketplace-reviewrecord** voor de officiële beoordelingsroute.

Die publieke onderdelen zijn volgens de huidige documentatie nog niet compleet. Deze repository doet daarom niet alsof er al een volledige externe productieruntime beschikbaar is.

### Waar begin je als ontwikkelaar?

Lees eerst de [manifestreferentie](../development/manifest-reference.md) voor de modulebeschrijving. Bekijk daarna de [publieke contracten](../../../contracts/README.md) voor de technische formaten en de [Conformance-tests](../../../conformance/README.md) om te begrijpen welke regels automatisch worden gecontroleerd.

<!-- section: visuals -->
## Diagrammen en screenshots

De diagrammen in deze documentatie tonen de gereviewde architectuur, niet noodzakelijk publiek beschikbare software. Er staan geen echte productiescreenshots in: bestaande Prism- en Core UI-schermen moeten eerst een privacyreview doorlopen. Mockups worden niet als werkende functionaliteit voorgesteld.
