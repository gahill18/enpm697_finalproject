# luciferris: a rusty proof of concept malware 🦀

## Roadmap

- [ ] Target Acquisition
  - [ ] Devices connected to the public internet
  - [ ] Devices connected to an already infected device on the local network
- [ ] Data Exfiltration
  - [X] Encrypt Files
  - [ ] Send keys/nonces to a landing point
    - [ ] Stand up landing site
    - [ ] Randomize landing site hardware and url
  - [ ] Send decryption key upon receipt of payment
- [ ] Steal Computing Power
  - [ ] Control/Reassign part or all of an infected device's computing resources
- [ ] Logging
  - [X] Resource Usage
  - [ ] Infection rates
  - [ ] Send logs to C&C
