# luciferris: a rusty proof of concept malware 🦀

## Roadmap

- [ ] Command and Control
  - [X] Infected devices communicate with hard coded C2 server
    - [X] Post logs
    - [X] Get commands/configs
  - [ ] C2 servers update and create new C2 servers, informing infected
- [ ] Target Acquisition
  - [ ] Devices connected to the public internet
  - [ ] Devices connected to an already infected device on the local network
- [ ] Data Exfiltration
  - [X] Encrypt Files
  - [ ] Securely send keys/nonces to C2
  - [ ] Send decryption key upon receipt of payment
- [X] Steal Computing Power
  - [X] Spawn tasks
- [ ] Logging
  - [X] Resource Usage
  - [ ] Infection rates
