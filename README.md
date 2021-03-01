# Market Simulator
A side project about a "simple" market simulator, originally made to help in a D&D campaign by giving some sort of realness to the market and to get comfortable with Rust.


## SQL Table Specifications
- Companies
  - id: unsigned small int
  - name: String
  - price: double
  - new_p: double
  - delta_p: double
  - volatility: double
  - bankrupt: bool
- ID_History
  - time
  - price
- Dependencies
  - in_id
  - out_id
  - weight

## TODO
- [ ] Decide solution to beginning time problem:
  - Solution 1:
    - Add beginning time field to Company struct and Companies Table
      - Pros:
        - State only depends on saved data, more specifically updates only depend on "local" data, easier to parallelize?
        - Bookkeeping is "simple", given a active company, current time = last time in ID_History (or equivalent) + company beginning time.
      - Cons:
        - It means the Company struct gets more complexity and it needs have more data.
        - More memory usage
    - Solution 2:
      - Add global time
        - Pros:
          - Super easy and light
          - Uses just one variable which only updates at the end of each update cycle.
        - Cons:
          - Non-intuitive place to save state in tables
          - Could give way to using a mutable global state, which could hurt parallelization?
- [ ] Finish io
  - [ ] Finish sql.rs
    - [ ] Check that sql.rs uses specified format (see SQL Table Specification)
  - [ ] Finish stdio.rs
    - [ ] Create format?
    - [ ] Specify format
    - [ ] Code specified format
- [ ] Redo main.rs
- [ ] Add documentation
- [ ] Check sim.rs "correctness" (i.e. specify expected behaviour in documentation and check it behaves as expected)
  - [ ] Solve spiral of death problem
  - [ ] Add queue of new companies? Or a mechanism to add created companies to the simulation in an automatic and natural way (polish and simplify idea)
- [ ] Add Events and Event log, which affect the market.
- [ ] Maybe add multiple markets and how they interact? (In a macro way, based on geographic distance or the time information travels from one market to another)
- [ ] Create a GUI?