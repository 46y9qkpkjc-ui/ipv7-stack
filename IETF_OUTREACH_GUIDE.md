# IETF Outreach Strategy & Email Templates

Complete guide for engaging with IETF leadership and working groups to drive IPv7 adoption.

---

## TABLE OF CONTENTS

1. [IETF Engagement Strategy](#ietf-engagement-strategy)
2. [Finding the Right Contacts](#finding-the-right-contacts)
3. [Email Templates](#email-templates)
4. [Mailing List Participation](#mailing-list-participation)
5. [IETF Meeting Preparation](#ietf-meeting-preparation)

---

## IETF ENGAGEMENT STRATEGY

### Phase 1: Outreach to Leadership (May 2026)

**Goal**: Get IPv7 on the radar of IETF area directors and working group chairs

**Timeline**:
- Week 1-2 (May 1-15): Research and contact area directors
- Week 3-4 (May 15-31): Engage in discussions on mailing lists
- June: Monitor responses and adjust approach

**Approach**:
1. **Respectful and Professional**: Position IPv7 as serious standardization effort
2. **Evidence-Based**: Reference the working implementation
3. **Open-Minded**: Invite feedback and criticism
4. **Persistent but Patient**: Don't expect immediate adoption

---

## FINDING THE RIGHT CONTACTS

### Step 1: Identify Relevant Area Directors

Visit: https://www.ietf.org/

Look for current IETF leadership (updates quarterly).

**Relevant Areas for IPv7**:

1. **Internet Area** (INT)
   - Responsibility: IP protocols, transport protocols
   - **Why**: IPv7 is a core IP protocol
   - Contact: Area Director (Internet)

2. **Routing Area** (RTG)
   - Responsibility: IP routing, BGP, OSPF
   - **Why**: IPv7 needs routing protocol support
   - Contact: Area Directors (Routing)

3. **Security Area** (SEC)
   - Responsibility: Security protocols and mechanisms
   - **Why**: SPV is a security mechanism
   - Contact: Area Director (Security)

4. **Operations and Management Area** (OPS)
   - Responsibility: Operations and deployment
   - **Why**: IPv7 has operational considerations
   - Contact: Area Directors (Operations and Management)

### Step 2: Find Their Contact Information

1. Go to: https://www.ietf.org/about/leadership/iesg/
2. Find the Area Director for each relevant area
3. Look for their email address (usually listed)
4. Note their name for personalization

### Step 3: Check Existing Working Groups

Visit: https://datatracker.ietf.org/wg/

Potential working groups that might be interested:

- **DRIP** (Drone Remote ID Protocol) - IoT security
- **RATS** (Remote Attestation Procedures) - Device authentication
- **CBOR** (Concise Binary Object Representation) - Serialization
- **QUIC** (QUIC Protocol) - Transport layer
- **TLS** (Transport Layer Security) - Encryption

**Action**: Check if any existing WGs have related charter

---

## EMAIL TEMPLATES

### Email 1: To Area Director (Internet Area)

**Subject**: IPv7 Internet-Draft - Working Group Consideration

**To**: [Area Director Name] <area-director-email@ietf.org>

**Body**:

```
Dear [Name],

I hope this email finds you well. I'm reaching out regarding draft-subbiah-ipv7-00, 
a Standards Track Internet-Draft I recently published on the IETF datatracker.

## Overview

IPv7 is a new network-layer protocol designed to address critical security gaps 
in current IP architecture, particularly regarding:

1. **Source Authentication**: Cryptographic binding between traffic origin and provider
2. **Proxy Mitigation**: First-hop validation to prevent unauthorized proxying
3. **Policy Enforcement**: Network-layer policy signaling for intent-based networking
4. **IoT Security**: Native mechanisms for device identity and trust

## Why Now?

The security landscape has changed dramatically since IPv6:
- Residential proxy networks ($B/year market) enable fraud and DDoS
- IoT devices (100M+) are commonly compromised and exploited
- Network-layer defenses are needed alongside application-layer controls

## Evidence of Viability

I've developed a complete reference implementation to demonstrate feasibility:
- 2,520 lines of production-grade Rust code
- 20/20 unit tests passing
- Comprehensive documentation for deployment and integration
- Performance: >50K packets/second (single thread)

Source: https://github.com/46y9qkpkjc-ui/ipv7-stack

## IETF Draft

Full specification: https://datatracker.ietf.org/doc/draft-subbiah-ipv7/

The draft includes:
- Complete technical specification (23 pages)
- Security analysis and threat modeling
- Use cases (botnet mitigation, interactive media, streaming)
- Deployment and transition considerations
- Implementation guidance

## Request

I'm seeking your guidance on:

1. **Relevance**: Does IPv7 align with the Internet Area's charter and priorities?
2. **Process**: What's the appropriate path for standardization?
3. **Feedback**: Are there specific concerns or areas needing strengthening?
4. **Adoption**: Would the Internet Area or a related WG be interested in this work?

## Timeline

- **Now (May 2026)**: Initial outreach and feedback gathering
- **June 2026**: Incorporate community feedback
- **August 2026**: Submit revised draft (v0.1)
- **September 2026**: Seek working group adoption

## Next Steps

I'm happy to:
- Present at IETF meetings or working group sessions
- Provide additional technical details or use case analysis
- Coordinate with other interested parties
- Address specific concerns or questions

Thank you for considering IPv7 for the IETF standards process.

Best regards,

Arunkumar Subbiah
Independent Contributor
arunkumar.subbiah@apexadversary.com
GitHub: https://github.com/46y9qkpkjc-ui/ipv7-stack

---

P.S. I've also published relevant discussions on the IETF mailing lists. Please feel 
free to reach out with any questions.
```

---

### Email 2: To Related Working Group Chair

**Subject**: IPv7 Consideration for [Working Group Name] Scope

**To**: [WG Chair Name] <wg-chair-email@ietf.org>

**Body**:

```
Dear [Name],

I'm reaching out regarding draft-subbiah-ipv7-00 and potential relevance to the 
[Working Group] working group.

## Relevance to [WG Name]

IPv7 has direct relevance to [WG] in the following areas:

[Customize based on WG - examples below]:

**For DRIP (Drone Remote ID)**:
- IPv7's identity-centric addressing could support drone identification
- SPV mechanism provides origin authentication for remote ID signals
- Trust levels enable safety-critical policy enforcement

**For RATS (Remote Attestation)**:
- Complements IPv7 for device authentication
- EIT mechanism integrates with attestation procedures
- Network-layer identity supports device posture verification

**For QUIC**:
- IPv7 provides complementary network-layer identity
- Could enhance QUIC's connection migration
- Trust signals inform QUIC-level policy decisions

## What We're Proposing

Rather than creating a new working group immediately, IPv7 could:
1. Be adopted by [WG Name] as part of your existing charter
2. Be liaised with if it remains independent
3. Be referenced as related work in your deliverables

## Implementation Status

Complete reference implementation demonstrates feasibility:
- https://github.com/46y9qkpkjc-ui/ipv7-stack
- Production-ready Rust implementation
- Comprehensive documentation

## Questions for You

1. Do you see IPv7 as within scope for [WG Name]?
2. What would be needed to consider adoption?
3. Are there related work items in your charter that should coordinate with IPv7?

## Next Steps

I'm available to:
- Present at your next WG meeting
- Provide detailed technical briefing
- Coordinate on interoperability
- Join your mailing list for discussion

Thank you for considering this proposal!

Best regards,

Arunkumar Subbiah
arunkumar.subbiah@apexadversary.com
https://datatracker.ietf.org/doc/draft-subbiah-ipv7/
```

---

### Email 3: To Security Area Director

**Subject**: IPv7 Security Mechanisms - SEC Review Coordination

**To**: [SEC Area Director] <area-director-email@ietf.org>

**Body**:

```
Dear [Name],

I'm writing to coordinate on the security mechanisms in draft-subbiah-ipv7-00.

## Security Components

IPv7 uses:
- **Ed25519** (NIST-approved): Digital signatures for origin validation
- **SHA-256**: Cryptographic hashing
- **Optional**: CRYSTALS-Dilithium for quantum-resistant future capability

All mechanisms follow NIST standards and are well-vetted.

## Security Analysis

The draft includes comprehensive security considerations:
- Signature key compromise scenarios
- Replay attack prevention
- Role escalation mitigation
- Privacy protection via Ephemeral Identity Tokens (EIT)
- Trust level manipulation defense

See Section 18 of: https://datatracker.ietf.org/doc/draft-subbiah-ipv7/

## SEC Area Perspective

I'd welcome feedback from the Security Area on:

1. **Cryptographic Choices**: Are Ed25519 + SHA-256 appropriate?
2. **Key Management**: Is the proposed key lifecycle adequate?
3. **Threat Model**: Does the security analysis cover relevant threats?
4. **Privacy**: Are privacy implications properly addressed?

## Process Question

Should IPv7 undergo security review through the SEC area, or is this handled 
differently for IP-layer protocols?

## Resources

- **Draft**: https://datatracker.ietf.org/doc/draft-subbiah-ipv7/
- **Implementation**: https://github.com/46y9qkpkjc-ui/ipv7-stack
- **Security Document**: See Section 18 of draft

Thank you for your guidance!

Best regards,

Arunkumar Subbiah
arunkumar.subbiah@apexadversary.com
```

---

## MAILING LIST PARTICIPATION

### Step 1: Subscribe to Key Lists

Visit: https://www.ietf.org/mailman/listinfo/

Subscribe to:

1. **ietf@ietf.org** (Main IETF list)
   - Subscribe at: https://www.ietf.org/mailman/listinfo/ietf
   - This is where most discussions happen
   - **Posting limit**: Some restrictions for new members

2. **general@ietf.org** (General IETF discussions)
   - More relaxed discussions
   - Good for technical questions

3. **int-discuss@ietf.org** (Internet Area discussion)
   - If IPv7 goes to Internet Area

Once subscribed:
- Confirm subscription via email
- Set email preferences (digest vs individual)
- Save the list archives

### Step 2: Introduction on Mailing List

After 1-2 weeks of lurking, introduce IPv7 respectfully:

**Subject**: [INT] Introducing draft-subbiah-ipv7-00

**Body**:

```
Hello IETF community,

I wanted to introduce draft-subbiah-ipv7-00, an Internet-Draft I recently 
published addressing residential proxy abuse and IoT security challenges.

## Quick Summary

IPv7 is a network-layer protocol adding:
- Identity-centric addressing (hierarchical identity strings)
- Cryptographic source-provider validation (Ed25519)
- Built-in trust/reputation signaling
- Three-stage router processing for policy enforcement

## Motivation

Residential proxy networks ($B/year market) enable fraud, credential stuffing, 
and DDoS by masking attacks behind legitimate home IPs. Current IPv4/v6 lack 
native origin authentication, leaving defense to expensive application-layer 
detection.

IPv7 shifts responsibility to the source network through cryptographic binding.

## Evidence of Viability

Complete reference implementation:
- 2,520 lines of production Rust
- 20/20 tests passing
- Comprehensive documentation
- https://github.com/46y9qkpkjc-ui/ipv7-stack

## Draft

https://datatracker.ietf.org/doc/draft-subbiah-ipv7/

## Seeking

Feedback on:
- Protocol design and approach
- Feasibility for standardization
- Appropriate working group home
- Technical concerns or improvements

Looking forward to discussing IPv7 with the community!

Best regards,
Arunkumar Subbiah
```

### Step 3: Engage in Discussions

- **Monitor responses** to your introduction
- **Respond to questions** with detailed, respectful answers
- **Don't spam** the list with repeated messages
- **Link to resources** rather than quoting extensively
- **Follow IETF mailing list culture** (professional, technical)

---

## IETF MEETING PREPARATION

### IETF 119 (November 2026)

This is your first in-person opportunity!

### Before the Meeting

**6 Months Before (May)**:
- [ ] Announce on IETF list that you'll be presenting
- [ ] Identify meetings you want to attend
- [ ] Register for IETF 119

**2 Months Before (September)**:
- [ ] Prepare 20-minute presentation slides
- [ ] Practice presentation (technical, not marketing)
- [ ] Request "lightning talk" or side meeting slot
- [ ] Finalize draft-subbiah-ipv7-01

**2 Weeks Before (October)**:
- [ ] Create printed handouts (optional)
- [ ] Finalize presentation
- [ ] Book flights/hotel

### At the Meeting

**What to Do**:
- [ ] Attend related working group sessions
- [ ] Present IPv7 (lightning talk/side meeting)
- [ ] Network with protocol designers
- [ ] Listen to feedback
- [ ] Collect business cards
- [ ] Schedule coffee chats with interested parties

**What NOT to Do**:
- Don't pitch aggressively
- Don't expect immediate working group adoption
- Don't argue about design decisions
- Don't dismiss feedback

### Presentation Outline (20 minutes)

```
1. Title slide (1 min)
   - Draft name and version
   - Your name and affiliation
   
2. Problem statement (3 min)
   - Residential proxies ($B/year market)
   - IoT device compromise
   - Current IPv4/v6 gaps
   
3. IPv7 solution (5 min)
   - Identity-centric addressing
   - Source-provider validation (SPV)
   - Trust/reputation signaling
   - Three-stage router pipeline
   
4. Implementation (3 min)
   - Reference implementation status
   - 20/20 tests passing
   - Link to GitHub
   
5. Use cases (2 min)
   - Botnet mitigation
   - Interactive media
   - Streaming video
   
6. Timeline (1 min)
   - Current status
   - v0.2.0, v0.3.0 plans
   - RFC path
   
7. Q&A (5 min)
   - Open discussion
   - Collect feedback
```

### Post-Meeting Follow-Up

**Within 1 Week**:
- [ ] Send thank-you emails to people you met
- [ ] Reference specific conversations
- [ ] Offer to send additional materials
- [ ] Follow up with action items discussed

**Within 1 Month**:
- [ ] Publish summary of IETF feedback
- [ ] Incorporate feedback into draft-01
- [ ] Continue mailing list participation

---

## OUTREACH CHECKLIST

### Phase 1: Email Outreach (Week 1-2 of May)

- [ ] **Internet Area Director**
  - Email sent: _______
  - Response received: _______
  - Follow-up needed: Yes / No

- [ ] **Routing Area Director**
  - Email sent: _______
  - Response received: _______
  - Follow-up needed: Yes / No

- [ ] **Security Area Director**
  - Email sent: _______
  - Response received: _______
  - Follow-up needed: Yes / No

- [ ] **3-5 Related WG Chairs**
  - Names: _____________________
  - Emails sent: _______
  - Responses: _____
  - Follow-ups: _____

### Phase 2: Mailing List (Week 3-4 of May)

- [ ] Subscribe to ietf@ietf.org
- [ ] Subscribe to general@ietf.org
- [ ] Subscribe to int-discuss@ietf.org
- [ ] Lurk for 1-2 weeks
- [ ] Post introduction
- [ ] Respond to initial feedback

### Phase 3: IETF Meeting (June onwards)

- [ ] Register for IETF 119 (November 2026)
- [ ] Prepare 20-minute presentation
- [ ] Request lightning talk/side meeting slot
- [ ] Network actively during meeting
- [ ] Collect feedback

### Phase 4: Working Group Adoption (December 2026+)

- [ ] Evaluate WG interest
- [ ] Coordinate with potential adopting WG
- [ ] Submit draft to WG (if adopted)
- [ ] Participate actively in WG discussions

---

## RESPONSE HANDLING

### If You Get Positive Response

```
Thank you for your interest in IPv7!

[Address specific points from their email]

I'd be happy to:
1. Provide more technical details
2. Present at a working group meeting
3. Coordinate on implementation details
4. Answer any questions

Looking forward to working with you!
```

### If You Get Critical Feedback

```
Thank you for the feedback. This is exactly the kind of constructive 
criticism we need.

[Address each point seriously]

I'll incorporate this into the next revision. Would it be helpful to:
1. Provide additional security analysis?
2. Discuss specific design trade-offs?
3. Share implementation results?

I appreciate you taking the time to engage with IPv7!
```

### If You Get No Response

- **Wait 2 weeks** before following up
- **Send brief follow-up**: "Following up on my IPv7 email from [date]..."
- **Don't be discouraged** - IETF leaders are busy
- **Try different contacts** if one doesn't respond
- **Be patient** - adoption takes time

---

## SUCCESS METRICS

**By End of June 2026**:
- [ ] 5+ emails sent to area directors/WG chairs
- [ ] 2+ positive responses received
- [ ] Joined 3+ IETF mailing lists
- [ ] Posted introduction on main list
- [ ] 5+ technical conversations started

**By End of September 2026**:
- [ ] Draft-subbiah-ipv7-01 submitted
- [ ] 2+ potential adopting WGs identified
- [ ] Regular participation on mailing lists
- [ ] Presentation prepared for IETF 119

**By End of November 2026**:
- [ ] Presented at IETF 119
- [ ] Met 10+ IETF participants
- [ ] Secured 1+ working group interest
- [ ] Collected feedback for future revisions

---

Good luck with IETF engagement! Remember: Be respectful, evidence-based, patient, and persistent. 🚀
