<!-- doc-key: contributing -->
<!-- parity-version: 3 -->
<!-- sections: first,issues,prs,rfc,modules,marketplace,licensing,rewards,bilingual -->

# Bijdragen aan Ivy

**Nederlands** | [English](../../en/community/contributing.md)

## Welkom!

Fijn dat je wilt helpen om Ivy beter te maken. Je hoeft Ivy niet vanbinnen en vanbuiten te kennen: een duidelijke foutmelding, een verbeterde uitleg of een kleine test kan al waardevol zijn.

Deze repository bevat de **publieke, experimentele Ivy Developer Kit**: hulpmiddelen, contracten en voorbeelden voor moduleontwikkelaars. Ze bevat niet de volledige private Ivy Core.

Je kunt bijdragen door een probleem te melden, een vraag te stellen, documentatie te verbeteren, tests toe te voegen of code voor te stellen. Hieronder lees je hoe dat werkt en welke afspraken daarbij gelden.

**Drie GitHub-begrippen om te kennen:**
- **Issue:** een melding van een bug of een afgebakend werkpunt.
- **Branch:** een aparte ontwikkellijn, zodat de bestaande code ongewijzigd blijft tijdens je werk.
- **Pull request (PR):** een voorstel om jouw wijzigingen na controle samen te voegen.

<!-- section: first -->
## Je eerste bijdrage

Begin bij voorkeur klein. Zo leren we het voorstel en de impact ervan kennen voordat er veel werk in kruipt.

1. Zoek een duidelijk afgebakend issue en laat in een reactie weten dat je eraan wilt werken, **voordat je veel tijd in de implementatie steekt**.
2. Maak een aparte branch voor die ene wijziging. Je kunt daarvoor ook vanuit je eigen fork werken.
3. Voer de controles uit vanuit de hoofdmap van de repository:

   ```bash
   ./scripts/check.sh
   ```

4. Open een pull request en beschrijf wat goed moet werken (**positieve tests**) én wat juist geweigerd of veilig afgehandeld moet worden (**negatieve tests**).

**Deel nooit private Ivy-broncode, wachtwoorden of tokens (credentials), persoonsgegevens of interne operationele informatie.** Ook niet in voorbeelden, screenshots, logs of PR-bijlagen.

<!-- section: issues -->
## Een fout melden of een vraag stellen

Heb je een fout gevonden in de Developer Kit? Maak dan een issue met genoeg informatie zodat iemand anders het probleem kan herhalen:
- de gebruikte versie of commit (*revision*);
- de kleinste reeks stappen of commando's om de fout na te bootsen;
- wat je verwachtte en wat er werkelijk gebeurde;
- je relevante omgeving, bijvoorbeeld besturingssysteem en gebruikte tools.

Voor gewone ontwikkelvragen gebruik je **GitHub Discussions → Q&A**, niet een bugissue.

**Beveiligingslek ontdekt?** Publiceer de details niet in een openbaar issue of Discussion. Gebruik **GitHub Private Vulnerability Reporting zodra de lanceringschecklist heeft bevestigd dat dit actief is**. Tot die bevestiging is er in deze handleiding nog geen geverifieerd privémeldkanaal aangewezen.

<!-- section: prs -->
## Een pull request indienen

Houd elke PR bij **één onderwerp**. Daardoor kunnen anderen de wijziging gemakkelijker begrijpen en beoordelen.

- Voeg passende tests toe en leg uit wat de wijziging aan publieke afspraken (*contracten*) verandert.
- Geef bij hergebruik van werk van anderen de herkomst en licentie aan.
- Werk bij publieke documentatiewijzigingen zowel de Nederlandse als Engelse versie bij.
- Laat architectuur- of beveiligingsgevoelige wijzigingen onafhankelijk beoordelen: de auteur keurt zijn eigen gevoelige wijziging niet goed.

Iedere commit moet worden ondertekend voor de **Developer Certificate of Origin (DCO) 1.1**:

```bash
git commit -s -m "Beschrijf de wijziging"
```

Dat voegt een `Signed-off-by`-regel toe: je bevestigt dat je het recht hebt om deze bijdrage onder de projectvoorwaarden in te dienen. Het is **geen overdracht van auteursrecht**. Meer informatie staat in [DCO](../../../DCO).

<!-- section: rfc -->
## Grotere ideeën eerst bespreken: RFC's

Een **RFC (Request for Comments)** is een ontwerpvoorstel dat anderen kunnen bespreken voordat er grote wijzigingen worden gebouwd.

Een RFC is nodig bij voorstellen die capabilities (functies), bevoegdheden (*authority*), vertrouwen (*trust*), compatibiliteit, pakketten of de Marketplace veranderen.

Beschrijf daarin:
- welk probleem je oplost;
- welke module of welk domein verantwoordelijk is (*ownership*);
- welke nieuwe of gewijzigde technische afspraak je voorstelt;
- wat gebeurt bij fouten en herhaalde aanvragen (*failure/retry*);
- hoe bestaande gebruikers kunnen overstappen (*migratie*) en hoe een wijziging teruggedraaid kan worden (*rollback*);
- welke alternatieven je onderzocht en afgewezen hebt.

Een kleine tekstcorrectie vraagt doorgaans geen RFC.

<!-- section: modules -->
## Zelf een Ivy-module maken

Een **module** voegt een afgebakende functie aan Ivy toe. Je kunt een zelfstandige module ontwikkelen zonder de broncode daarvan aan ANWOR over te dragen.

Beschrijf in je voorstel:
- het domein waarvoor de module verantwoordelijk is, en expliciet wat ze **niet** beheert;
- welke functies ze aanbiedt (*provided capabilities*);
- welke functies van anderen ze nodig heeft (*required capabilities*);
- voorbeelden waarmee geldige en ongeldige situaties worden getest (*conformancefixtures*).

Gebruik daarvoor het **moduleproposal-template**. Bewaar zelfstandige modulecode in je **eigen repository**, tenzij ANWOR uitdrukkelijk aanvaardt dat ze als gedeelde referentiecode in deze Developer Kit wordt opgenomen.

<!-- section: marketplace -->
## Een module aanbieden in de Ivy Marketplace

Een werkende module verschijnt **niet automatisch** in de officiële Marketplace. Voor opname volgt een aparte beoordeling van de precieze module-identiteit, versie en inhoudsvingerafdruk (*digest*).

De labels `ANWOR Official` en `ANWOR Verified` zijn beoordelingen, **geen overdracht van eigendom**. Ze gelden ook niet automatisch voor gewijzigde modules of latere versies.

<!-- section: licensing -->
## Van wie blijft de code?

**Je eigen zelfstandige module blijft van jou.** Als externe auteur kies je zelf of je die gratis, commercieel, gesloten (*proprietary*) of open source aanbiedt, voor zover je daar de rechten voor hebt.

**Code die wordt aanvaard als bijdrage aan deze gedeelde Developer Kit valt onder Apache-2.0.** Hiervoor gebruiken we DCO 1.1; een afzonderlijke Contributor License Agreement (**CLA**) is niet vereist.

De code-licentie geeft **geen** gebruiksrechten op de namen, logo's, handelsmerken of officiële badges van Ivy of ANWOR. Zie [LICENSE](../../../LICENSE) en [DCO](../../../DCO).

<!-- section: rewards -->
## Erkenning en beloningen

ANWOR kan per bijdrage afzonderlijk erkenning, credits, een betaling of een andere beloning afspreken. Dat is geen automatische vergoeding: het indienen of aanvaarden van een bijdrage schept op zichzelf geen recht op betaling.

<!-- section: bilingual -->
## Documentatie in twee talen

We willen dat Nederlandstalige en Engelstalige ontwikkelaars dezelfde informatie krijgen.

Wijzig je publieke documentatie voor mensen? Pas dan **in dezelfde PR** zowel het `nl`- als het overeenkomstige `en`-bestand aan. Houd de interne paritymetadata en sectie-ID's gelijk en behoud werkende codevoorbeelden.

De automatische CI-controle weigert ontbrekende vertalingen, niet-overeenkomende secties, verschillen in commando's (*command drift*) en kapotte interne links.
