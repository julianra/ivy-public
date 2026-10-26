<!-- doc-key: manifest-reference -->
<!-- parity-version: 2 -->
<!-- sections: scope,syntax,semantics,fixtures,generator -->

# Canonieke publieke manifestsubset

**Nederlands** | [English](../../en/development/manifest-reference.md)

<!-- section: scope -->
## Scope

Het publieke schema is een strikte developergerichte subset van private
manifestschema versie 3. Het laat productievelden voor runtime, IPC, sandbox,
platform, resources, distributie en permissions bewust weg.

<!-- section: syntax -->
## Structurele regels

- module-ID: lowercase ASCII-woorden gescheiden door `-`;
- namespace-/domeinsegmenten: beginnen met een lowercase letter en bevatten
  lowercase letters, cijfers of `_`, uitsluitend gescheiden door `.`;
- namespace/domein: minstens twee segmenten;
- capability-ID: minstens drie segmenten;
- versie: SemVer 2.0.0;
- schemareferentie: `schema:<naam-met-minstens-twee-segmenten>:v<positief-getal>`;
- commands vereisen `retry_semantics`; queries verbieden dit;
- alle velden zijn expliciet en onbekende velden falen validatie.

<!-- section: semantics -->
## Semantische regels alleen in Rust

JSON Schema kan niet elke invariant uitdrukken. Rust weigert aanvullend dubbele
capability-ID's, overlappende domeinen binnen één manifest en provided
capabilities buiten een owned domain. De simulator controleert conflicten met
geregistreerde modules en exacte handlerdekking voordat registratie atomisch
wordt toegepast.

<!-- section: fixtures -->
## Gedeelde fixtures

Elke fixture doorloopt beide paden. `schema-invalid` moet in beide falen;
`semantic-invalid` moet structureel slagen maar semantisch falen; `valid` moet in
beide slagen. Zo kunnen schema, parser, voorbeelden en documentatie niet
stilzwijgend uit elkaar lopen.

<!-- section: generator -->
## Generator

De CLI maakt alleen manifests die de canonieke validator accepteert.
Gegenereerde skeletons zijn developmentstartpunten, geen installeerbare
Ivy-packages.
