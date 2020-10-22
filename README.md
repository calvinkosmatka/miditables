# miditables

ALSA MIDI router with iptables style syntax

#### 1.0 Roadmap
- [ ] Basic set of built-in matchers/transformers
  - [ ] Matchers
    - [x] Input port
    - [ ] Event type
    - [ ] Event data
      - [ ] Note data
        - [ ] Channel
        - [ ] Note
        - [ ] Velocity
        - [ ] Aftertouch
        - [ ] For all of the above, allow:
          - [ ] Single values
          - [ ] Ranges
      - [ ] Control data
        - [ ] Channel
        - [ ] CC
        - [ ] Value
        - [ ] Pitchbend
        - [ ] Channel Pressure
        - [ ] For all of the above, allow:
          - [ ] Single values
          - [ ] Ranges
  - [ ] Transformers
    - [x] Print event to console (debug)
    - [ ] Output on particular port
    - [ ] Manipulate event data
      - [ ] Note data
        - [ ] Channel
        - [ ] Note
        - [ ] Velocity
        - [ ] Aftertouch
        - [ ] For all of the above, allow:
          - [ ] Change to a fixed value
          - [ ] Offset by a fixed value (wrap or saturate)
          - [ ] Multiply by a fixed value
          - [ ] Modulo (fixed modulus
      - [ ] Control data
        - [ ] Channel
        - [ ] CC
        - [ ] Value
        - [ ] Pitchbend
        - [ ] Channel Pressure
        - [ ] For all of the above, allow:
          - [ ] Change to a fixed value
          - [ ] Offset by a fixed value (wrap or saturate)
          - [ ] Multiply by a fixed value
          - [ ] Modulo (fixed modulus)
- [ ] Stable miditables.conf syntax
  - [x] #inputs / #outputs
  - [x] Table/Chains
    - [x] -N [chain name] to create new chain
    - [x] -A [chain name] [rule spec] to add rule to chain
  - [ ] Rules
    - [x] -m [matcher name] to declare matcher
    - [ ] -m [plugin namespace]:[matcher name] to declare matcher from plugin
    - [x] -t [transformer name] to declare transformer
    - [ ] -t [plugin namespace]:[transformer name] to declare transformer from plugin
    - [ ] -j [jump type] to specify post-transform action for matched events (jump)
      - [x] -j [chainx] to process the event in named chainx
      - [ ] -j END to stop processing event (similar to -j DROP in iptables)
      - [ ] -j CONTINUE (optional) to continue processing in current chain
    - [ ] -!m [matcher name] to negate matcher
  - [ ] Plugins
    - [ ] #plugin [path to .so] [namespace] 
  - [ ] Global matchers/transformers
- [ ] Stable public interface
- [ ] Stable plugin ABI
- [ ] Multithreading (thread for each input event)
  - [ ] Determine strategy

#### Beyond 1.0
- [ ] #include directive for miditables.conf
- [ ] User (and system) defined miditables.conf macros
- [ ] Support more than just ALSA
  - [ ] Look into using midir
- [ ] Allow transformers to dispatch new events
