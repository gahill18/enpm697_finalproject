# luciferris: a rusty proof of concept malware 🦀

## Roadmap

- [ ] Target Acquisition
  - [ ] Devices connected to the public internet
  - [ ] Devices connected to an already infected device on the local network
- [ ] Data Exfiltration
  - [ ] Send readable file contents to a landing point
    - [ ] Stand up landing site
    - [ ] Randomize landing site hardware and url
  - [ ] Encrypt files that have been exfiltrated
  - [ ] Send decryption key upon receipt of payment
- [ ] Steal Computing Power
  - [ ] Control/Reassign part or all of an infected device's computing resources
- [ ] Logging
  - [ ] Log infection rates and resource usage
