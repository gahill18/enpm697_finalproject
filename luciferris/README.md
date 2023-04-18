# luciferris: a rusty proof of concept malware 🦀

## Roadmap

- [ ] Target Acquisition
  - [ ] Devices connected to the public internet
  - [ ] Devices connected to an already infected device on the local network
- [ ] Data Exfiltration
  - [X] Encrypt Files
  - [ ] Securely send keys/nonces to a landing point
  - [ ] Send decryption key upon receipt of payment
- [X] Steal Computing Power
  - [X] Control/Reassign part or all of an infected device's computing resources
- [ ] Logging
  - [X] Resource Usage
  - [ ] Infection rates
  - [ ] Send logs to C&C
- [ ] Command and Control
  - [ ] Infected devices communicate with hard coded C2 server
  - [ ] C2 servers update and create new C2 servers, informing infected