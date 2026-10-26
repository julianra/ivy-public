# Ivy publication matrix

Audited against private source commit `453b2bea65c7765419ac1a11e4185479b2234c1b`
on 2026-10-26. `Pending` means no final public licence has been approved.

| Component | Purpose | Public suitability | Included now | Licence state | Required work | Independent build/test |
|---|---|---|---|---|---|---|
| Core | Registry, grants, routing, lifecycle, audit, package trust | Architecture only | No source | Proprietary | Sanitised docs only | No; intentionally private |
| Core UI | Local platform administration | Screenshots/docs only | No source | Proprietary | Screenshot privacy review | No; depends on Core |
| Prism | Generic UI and widget composition | Selected client/renderer code later | Concepts only | System-module licence pending | Remove release config, private endpoints and mobile trust details | Not in this repo |
| Bridge | External network and channel boundary | High-risk; docs only | Architecture summary | System-module licence pending | Dedicated security and licence review | Not in this repo |
| Vault | Durable storage and key-protected state | Security-sensitive | Contract concepts only | System-module licence pending | Crypto/storage review | Not in this repo |
| Hooman | People and identity authority | Security-sensitive | Concepts only | System-module licence pending | Identity/security review | Not in this repo |
| Flow | Durable routines, retries and approvals | Possible later reference implementation | Concepts only | System-module licence pending | Stabilise API and decide IP boundary | Not in this repo |
| Pulse | Time-management domain | Possible later module | Description only | Community/system classification pending | Licence and API review | Not in this repo |
| Agenda | Calendar, availability and time model | Strong later reference module | Description only | Community-module licence pending | Sanitise and separate storage integration | Not in this repo |
| BookMe | Booking composition | Useful case study later | Description only | Community-module licence pending | Reconcile historical docs and dependencies | Not in this repo |
| Rust platform SDK | Private wire/runtime helpers | Only a narrow public surface is suitable | Reimplemented public subset | Apache-2.0 proposed | Rights and API review | Yes |
| Dart SDK | Private generated runtime/mobile types | Narrow developer types suitable | New independent types | Apache-2.0 proposed | Add reproducible proto generation | Yes |
| Private ivy-cli | Build/sign/admin CLI; directly imports Core | Not suitable wholesale | No | Proprietary/pending mix | Extract public commands only | Original cannot build without Core |
| Public ivy-cli | Generate, validate and dev-sign | Suitable | Yes | Apache-2.0 proposed | Legal approval and UX hardening | Yes |
| Widget preview | Local fixture rendering | Suitable after separation | New offline implementation | Apache-2.0 proposed | Align with stable widget vocabulary | Yes |
| Runtime/mobile protobuf | Internal runtime and authenticated mobile wire | Too broad for initial release | No | Rights review required | Split developer-facing protocol | No |
| Public developer proto | Capability declaration and test envelope | Suitable | Yes | Apache-2.0 proposed | Add code generation | Schema check only |
| Manifest v3 | Installation and capability declaration | Public subset suitable | JSON Schema plus Rust semantic validation | Apache-2.0 proposed | Confirm subset compatibility | Yes |
| Package signing | Official provenance and protected keys | Concepts public; operations private | Development-only signer | Mixed; official system proprietary | Separate package format and threat review | Dev signer yes |
| Conformance | Validate module behavior | Suitable | Initial suite | Apache-2.0 proposed | Add events, package layout and failures | Yes |
| hello-clock | Minimal provider/grant example | Suitable | Yes | Apache-2.0 proposed | External clean-clone validation | Yes |
| Marketplace metadata | Version-specific provenance/review | Suitable as spec later | Policy docs only | Policy/legal approval pending | Define signed verification record | Documentation only |
| Juma/Attraxi code and migration | Business applications and operations | Not suitable for platform repo | No | Separate rights | Case studies only after sanitisation | No |
| Internal roadmap, workers, Manager/Orchestrator | Development operations | Not a public product surface | No | Proprietary/internal | Publish methodology only | No |
