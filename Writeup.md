# Malware Development from Design to Deployment

By Garrett Hill

## Table of Contents

1. [What is Malware?](#whatismalware)
2. [Malware Development Lifecycle](#mdlc)
3. [Demonstration](#demo)
4. [Conclusions](#conclusions)
4. [Resources](#resources)

## What is Malware? <a name="whatismalware"></a>

Malicious software, or "malware", is a broad array of programs and scripts intended to cause damage or acquire unauthorized access.[1]
It can be used in pursuit of a variety of interests, including:

- financial gain
- revenge
- ideology
- ego/reputation
- recreation (fun!)

According to Forbes, malware cost the world $20 billion USD in 2021, and that number is projected to rise to $265 billion annually by 2031.
These cost estimates are derived from many elements including destroyed/damaged resources, lost productive business hours, and sullied reputations.

Malware also comes in many different forms. There are three primary overlapping categories:

- Trojans, or malware disguised as real software/files to trick users into installing it themselves
- Worms, or malware that spreads itself across a network without human intervention
- Ransomware, or malware that locks down resources on a device until a payment (the ransom) is paid to unlock them

### Who makes it it and why is the development process important?

In its infancy, malware was largely made by hobbyists and those seeking personaly glory amongst fellow hackers. However, the modern malware developer usually falls into one of three groups:

1. Nation-state sponsored
2. Malware as a Business
3. Security Researchers *Predicting* Future Malware

There is an ongoing arms race between these three groups. Each one has both conflicting and overlapping interests, and they are each fighting against the others.
The proliferation of Malware as a Service (MaaS), and more specifically Ransomware as a Service (RaaS) is inextricably linked to the rise of the professional hacker group.
These services are sold to anyone looking to utilize them, meaning anyone with $5000 USD and a motive can access top of the line ransomware and target a place of business they are personally familiar with.
Additionally, the rise in cyber warfare has introduced the world to some of the most advanced malware ever, like the US government's Stuxnet and WannaCry.

So, knowing that malware is immensely profitable and vital to modern warfare, it seems valuable to be able to predict how both current and future malware works.

## The Malware Development Lifecycle <a name="mdlc"></a>

The Malware Development Lifecycle (MDLC) can be broken up into 6 main sections:

1. Target
2. Objectives
3. Research
4. Implement
5. Deploy
6. Maintain

### Target

All malware is created with a purpose, just like any other piece of software. Malware needs to be designed around the environment it will be deployed to, which includes analysis of both hardware and software.
It might not be enough to know what operating system your target is running, you may need to know what architecture (hardware) it is running on.
For example, malware targeting x86 processors might be different from that targeting ARM architecture.

### Objectives

With how many different services run digitally these days, entire sectors of national economies are reliant on server uptime.
As mentioned previously, the nation state actor will generally have a different objective than the MaaS provider, but all malware will generally have at least one of two motives:

1. Political motives (more aligned with the nation state actor)
2. Financial motives

Political motives are fundamentally focused on information and control of resources. Infrastructure, defense technology, and state secrets are primary targets, and espionage or sabotage are much more prevalent.
Financial motives are fundamentally focused on making as much money as possible over the course of a malware's lifespan. Ransomware, extortion/blackmail, and identity theft are some of the most common methods.
These motives drive our malware's methodology. If we want to steal a set of emails from a political party's private server, our methods will look very different to if we wanted to sabotage an oil pipeline.

### Research

With our target and objectives defined, we can finally start digging into technical details.
The most advanced malware might be custom designed for a given target's network, but more often malware is designed with generality in mind.
If we know that our target is an Ubuntu server running on an x86 chipset running a Jellyfin server, we can start googling relevent CVEs that might not have been patched yet.

### Implement

### Deploy

### Maintain

## Demonstration! <a name="demo"></a>

### Target

Before choosing what we want out of our malware, we need to choose our attack space. Knowing what systems we are attacking allows us to make informed design decisions in our exploitation.

- [Primary] Web servers running Ubuntu/Debian-based distributions of Linux
- [Secondary] Web servers running Fedora/RedHat/CentOS-based distributions of Linux
- [Ternary] Other devices visible to the public internet with vulnerable access points

The first two systems are quite similar, but use different software managemers (apt vs dnf) and different kernels. This will impact the number and type of vulnerabilities somewhat, but as long as the target device
has already installed the vulnerable dependencies, we can attack them in almost exactly the same ways.

The third system(s) are very generic, and serves only as a guide for future research.

### Objectives

Knowing our attack space, we can decide what we want our malware to do:

1. Partially automate target acquisition via web scraping and local network callouts
2. Access/Exfiltrate information
3. Control/Reassign part or all of an infected device's computing resources
4. Log infection rates and resource usage

### Target Research

What do we know about web servers?

### Choosing an Exploit

### Implementation

### Deployment

### Results

## Conclusions <a name="conclusions"></a>

## Resources <a name="resources"></a>

1. https://www.researchgate.net/publication/358979335_Malware_development
2. https://www.lifewire.com/brief-history-of-malware-153616
3. https://www.comparitech.com/antivirus/malware-statistics-facts/
4. https://www.forbes.com/sites/chuckbrooks/2022/01/21/cybersecurity-in-2022--a-fresh-look-at-some-very-alarming-stats/?sh=3282430d6b61