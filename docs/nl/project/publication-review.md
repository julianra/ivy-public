<!-- doc-key: publication-review -->
<!-- parity-version: 3 -->
<!-- sections: legal,product,community,security,github,launch -->

# Publicatiereview

**Nederlands** | [English](../../en/project/publication-review.md)

De repository moet private blijven totdat elke blocker is opgelost en de
eigenaar afzonderlijk expliciete publicatietoestemming geeft.

<!-- section: legal -->
## Juridische beslissingen

- [x] Pas Apache-2.0 toe op de Ivy Developer Kit in deze repository.
- [x] Gebruik DCO 1.1 voor bijdragen; geen CLA.
- [x] Pas repositorybeleid voor trademarks/badges en de NOTICE-grens toe.
- [ ] Bevestig eventuele eigendomsspecifieke copyrightvermelding.
- [ ] Keur System Module- en Community Module-licenties goed.

<!-- section: product -->
## Productbeslissingen

- [ ] Keur publieke organisatie-/repositorynaam goed.
- [ ] Keur compatibiliteitsbelofte en stabiliteit van de manifestsubset goed.
- [ ] Kies de eerste publieke referentiemodule.
- [ ] Beslis of publieke onderdelen hier canonical worden.
- [ ] Keur beleid voor `ANWOR Official` versus `ANWOR Verified` goed.

<!-- section: community -->
## Communitybeslissingen

- [ ] Kies definitieve standaard-Code of Conduct en privécontacten.
- [ ] Benoem maintainers en CODEOWNERS.
- [ ] Keur Discussioncategorieën, responstermijnen en beloningsproces goed.

<!-- section: security -->
## Security en privacy

- [ ] Onafhankelijke scan en menselijke review van exacte launchcommit en historie.
- [ ] Menselijke review van elke screenshot/media-asset.
- [ ] `MANUAL GITHUB SETTING REQUIRED`: activeer en verifieer GitHub Private Vulnerability Reporting.
- [ ] Herhaal clean-clonebuilds zonder toegang tot de private repository.

<!-- section: github -->
## GitHub-controls

- [x] `VERIFIED ACTIVE`: CI, Gitleaks, repository-safety en Dependabot-securitycontrols.
- [x] `READY WHEN PUBLIC`: CodeQL- en Dependency Review-workflows.
- [ ] `MANUAL GITHUB SETTING REQUIRED`: branch rules met review, CODEOWNERS, CI en zonder force-push/delete.
- [ ] `MANUAL GITHUB SETTING REQUIRED`: native secret scanning en push protection.
- [ ] Bevestig nul open geautomatiseerde PR's en geen extra branch.

<!-- section: launch -->
## Launchbevoegdheid

- [ ] Repository-eigenaar geeft een afzonderlijke expliciete opdracht om
      visibility van PRIVATE naar PUBLIC te wijzigen.

Geen release, aankondiging, packagepublicatie of visibilitywijziging is toegestaan
vóór het laatste punt.
