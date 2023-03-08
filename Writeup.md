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
That's where the third group comes in. Security researchers need to know WHAT malware will look like, so they want to know HOW it is built. Let's investigate the how!

## The Malware Development Lifecycle <a name="mdlc"></a>

The Malware Development Lifecycle (MDLC) can be broken up into 6 main sections:

1. Target
2. Objective(s)
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

With a target and objectives defined, we can finally start digging into technical details.
The most advanced malware might be custom designed for a given target's network, but more often malware is designed with generality in mind.
If we know that our target is an Ubuntu server running on an x86 chipset running a Jellyfin server, we can start googling relevent CVEs that might not have been patched yet.

### Implement

So we've identified a vulnerability we want to exploit. Since malware is just software, we need to write some code that will interact with our target. That requires picking a "tech stack". There are countless choices, so how do we narrow it down?

Look for a two main things when picking a malware development stack:

1. Easy (enough) to Write
2. Easy (enough) to Deploy

Some languages might be easy to write due to their simple syntax and familiar data structures, but might be very difficult to deploy under expected operating conditions.
Python is an incredibly popular language due to how quick you can write prototype functionality, but as an interpreted language, it requires an interpreter to be installed and run on your target machine, which is vulnerable to being caught early.
On the other hand, languages like C++, Nim, and Rust are compiled languages, which means their functionality is baked down into a single executable with no external dependencies (usually).
However, these executable files are only compatible with the architecture type they are explicitly compiled for, meaning you need multiple executables if you are targetting different system architectures.

Once you've picked a language, you can get to work. You can look for common libraries or existing projects that might implememt some of the functionality you're looking for, but be careful to obfuscate any copy/pasted source code to avoid antivirus flagging known hash values.
It may be worth reimplementing any functionality you intend to borrow independently to avoid this.

Implementing basic functionality may not take a significant amount of time if relying on simple exploits, but some nation-state level development can occur for months or years.
Test frequently against industry-common virus detection programs to see if your work is being flagged. Obfuscation can take even longer than core functionality development, so do not wait until the end of the project to start obfuscating.
Document internally, but do not share your work publicly, for obvious reasons.

### Deploy

Once you've tested rigorously against your mock target/targets, you can begin your infection campaign. This will look different for different kinds of malware;
Your first victim might need to click a malicious link in a phishing email to download and install the program on their computer, or maybe you have an insider carry the program in on a USB.
After the initial infection, the spread will depend heavily on your implementation. Perhaps you send another phishing email to every saved email contact on the infected device, or maybe you work your way quietly through all the connected devices (printers, speakers, etc).

### Maintain

With your malware out in the world, you at minimum need a way to keep track of it's mission status. One way to do this is by using a command-and-control (C&C) server, which is responsible for receiving reports and issuing new commands to devices infected with your malware.[5]
If you are receiving ransom payments for decryption keys, you need to have those keys on file somewhere accessible to only you, and you need to be able to actually receive payments in a way that can't be traced back to you.
Payments are usually done with cryptocurrencies, but not all currencies are created equal. Each has different levels of anonymity and security, with Monero being the "Privacy Coin" with the largest current market cap.[6]

## Demonstration! <a name="demo"></a>

### Target

Before choosing what we want out of our malware, we need to choose our attack space. Knowing what systems we are attacking allows us to make informed design decisions in our exploitation.

- [Primary] Web servers running Ubuntu/Debian-based distributions of Linux
- [Secondary] Web servers running Fedora/RedHat/CentOS-based distributions of Linux
- [Ternary] Other devices visible to the public internet with vulnerable access points

The first two systems are quite similar, but use different package managemers and different kernels. This will impact the number and type of vulnerabilities somewhat, but as long as the target device has already installed the vulnerable dependencies, we can attack them in almost exactly the same ways.
The third system(s) are very generic, and serves only as a guide for future research.

### Objectives

Knowing our attack space, we can decide what we want our malware to do:

1. Partially automate target acquisition via web scraping and local network callouts
2. Read/Exfiltrate/Encrypt Files
3. Control/Reassign part or all of an infected device's computing resources
4. Log infection rates and resource usage

### Target Research

What do we know about web servers?

1. Web servers run applications exposed to the internet
2. Improper configurations introduce vulnerabilities
3. Web servers are often improperly configured

Ubuntu, a Debian and GNU/Linux based OS, is distributed as a server OS by the Canonical company. It supports many different architecture formats, including ARM, x86, and IBM POWER.
In addition to bare-metal support, Ubuntu is also distributed as a Virtual Machine for testing and cloud-based distributed computing.
For this demonstration, we will focus primarily on the Ubuntu 22.04.2 Long Term Service (LTS) for AMD/x86 distribution.
We do so because it is one of the most popular OS on the web, and has extensive community documentation for security vulnerabilities.

Because we have access to the same installation image as the users setting up the target devices, we know the exact starting state of their systems.
We can safely assume that at least a small portion of users will *never change the default configurations*, and of those who do change their configurations, a portion will configure their system in an unsecure way.[7]
It is no longer enough to disable unused ports, or to change your password off of the default. Requiring public key authentication for SSH connections, maintaining a healthy firewall, and many other best practices are now necessary.

### Choosing an Exploit

Knowing that even a small portion of our target demographic will be improperly configured means we can rely on a laundry list of known configuration vulnerabilities and achieve vast numerical success.
After all, 0.1% of 10,000,000 is still 10,000 infected devices. We will focus on the following common configuration vulnerabilities:

- Default Login Credentials
- Credentials stored in plain text/Visibly hard coded
- Lack of login attempt rate limiting
- Improper file access permissions
- Credentials transmitted unencrypted

### Implementation

As mentioned above, we want to consider ease of development and ease of deployment when choosing how to implement our configuration vulnerability exploitation code.
There are pros and cons to different languages, but for this project, I will be using the Rust language.
I chose Rust mostly because I am familiar and comfortable with it already, but also because it has an ever growing community functionality library, which allows me to avoid reinventing the wheel in many places.
While a professional malware team might implement all functionality from scratch, I don't have the skills or time to accomplish that within one semester.

With a language chosen, I started the first part of development: roadmapping. Using the stated objectives from above, I came up with the following work tree:

[ ] Target Acquisition
  [ ] Devices connected to the public internet
  [ ] Devices connected to an already infected device on the local network
[ ] Data Exfiltration
  [ ] Send readable file contents to a landing point
    [ ] Stand up landing site
    [ ] Randomize landing site hardware and url
  [ ] Encrypt files that have been exfiltrated
  [ ] Send decryption key upon receipt of payment
[ ] Steal Computing Power
  [ ] Control/Reassign part or all of an infected device's computing resources
[ ] Logging
  [ ] Log infection rates and resource usage

With a general outline of functionality established, I started searching for preexisting library functionality I could lean on. Below is a list of libraries, or "crates" as Rust calls them, that I utilize, along with what I use them for.

- [config](https://crates.io/crates/config) (Configuration)
- [shodan](https://crates.io/crates/shodan) (Internet scanning)
- [qscan](https://crates.io/crates/qscan) (Local network scanning)
- [sysinfo](https://crates.io/crates/sysinfo) (Host system information)
- [_]() (File system analysis)
- [rustls](https://crates.io/crates/rustls) (Outbound messages)
- [chacha20poly1305](https://crates.io/crates/chacha20poly1305) (Encryption)
- [env_logger](https://crates.io/crates/env_logger) (Logging)

### Deployment

### Results

## Conclusions <a name="conclusions"></a>

## Resources <a name="resources"></a>

1. https://www.researchgate.net/publication/358979335_Malware_development
2. https://www.lifewire.com/brief-history-of-malware-153616
3. https://www.comparitech.com/antivirus/malware-statistics-facts/
4. https://www.forbes.com/sites/chuckbrooks/2022/01/21/cybersecurity-in-2022--a-fresh-look-at-some-very-alarming-stats/?sh=3282430d6b61
5. https://www.techtarget.com/whatis/definition/command-and-control-server-CC-server
6. https://decrypt.co/resources/what-are-privacy-coins-monero-zcash-and-dash-explained
7. https://www.datto.com/blog/what-is-a-configuration-vulnerability