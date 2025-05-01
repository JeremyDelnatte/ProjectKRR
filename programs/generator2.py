import clingo
import secrets

facts = []
seed = None
# filter = ["pipe_unrotated", "cell_value"]

def collect_model(model):
    global facts
    facts.extend([str(atom) + "." for atom in model.symbols(atoms=True)])

def print_model(model: clingo.Model):
    model_str = [atom + "." for atom in str(model).split(" ")]
    print("\n".join(model_str))

def solve_step(program, step, rand_freq=1.0, parallel=11, last=False, parameters=None):
    if parameters is None:
        parameters = []

    ctl = clingo.Control(["--rand-freq=" + str(rand_freq), "-t " + str(parallel), f"--seed={str(seed)}"] + parameters)
    ctl.load(program)
    ctl.add("facts", [], "\n".join(facts))
    ctl.ground([("facts", []), (step, [])])

    if last:
        ctl.solve(on_model=print_model)
    else:
        ctl.solve(on_model=collect_model)

def run_generator(height, width, depth, print=False):
    global facts
    facts = []
    program = "generator2-programs.lp"
    global seed
    seed = secrets.randbelow(2**31)

    solve_step(program, "init", rand_freq=0, parallel=1, parameters=["-c h=" +str(height), "-c w=" + str(width), "-c d=" + str(depth)])
    solve_step(program, "block_size_gen", rand_freq=1, parallel=1)
    solve_step(program, "block_gen", rand_freq=1, parallel=11)
    solve_step(program, "pipe_gen", rand_freq=1, parallel=11)
    solve_step(program, "rotation", rand_freq=1, parallel=11, last=print)

def run_solver(print=False):
    program = "solver.lp"
    filter = ["width", "height", "depth", "block", "pipe", "pipe_in", "pipe_out"]
    filtered = [item for item in facts if any(item.startswith(f"{f}(") for f in filter)]

    ctl = clingo.Control(["-t 11"])
    ctl.load(program)
    ctl.add("facts", [], "\n".join(filtered))
    ctl.ground([("facts", []), ("base", [])])

    if print:
        ctl.solve(on_model=print_model)
    else:
        ctl.solve()

if __name__ == "__main__":
    run_generator(3, 3, 3, print=False)
    print("Generator finished")
    run_solver(print=False)
