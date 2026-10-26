<!-- doc-key: marketplace-verification -->
<!-- parity-version: 2 -->
<!-- sections: purpose,record,process,labels,limits -->

# Marketplaceverificatie

**Nederlands** | [English](../../en/marketplace/verification.md)

<!-- section: purpose -->
## Doel

Marketplace-review communiceert provenance en reviewstatus zonder eigendom van
een externe ontwikkelaar over te nemen.

<!-- section: record -->
## Versiegebonden record

Goedkeuring moet namespace en ID, exacte versie, packagedigest,
signer/provenance, reviewpolicyversie, reviewresultaat, timestamp en ondersteund
Ivy-bereik binden.

<!-- section: process -->
## Voorgesteld proces

1. De auteur levert onveranderlijke package-identiteit en rechteninformatie.
2. ANWOR controleert gedeclareerde authority, dependencies, security en compatibiliteit.
3. Het exacte resultaat wordt vastgelegd en bij goedkeuring ondertekend.
4. Elke gewijzigde package of nieuwe versie vereist een nieuwe review.

<!-- section: labels -->
## Labels

`ANWOR Official` is voorbehouden aan gecontroleerde officiële releases.
`ANWOR Verified` identificeert een exact gereviewde package van derden. Geen van
beide labels gaat automatisch over op forks of toekomstige versies.

<!-- section: limits -->
## Huidige beperkingen

De Marketplace-implementatie, reviewpolicy, juridische voorwaarden en het
ondertekende recordformat zijn gepland. Deze repository verleent geen certificering.
