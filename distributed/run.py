import subprocess
import os
import threading
import time

username = "marcusl"


def run_here(na, nb, nt):
    subprocess.run("~/programming/yatzy_solver/solver "
                   f"compute-strat-6x {na} {nb} {nt}",
                   shell=True)


def run_remote(machine, na, nb, nt):
    if na < 6:
        subprocess.run(
            f"scp /scratch/{username}/cache/6x/scores/{na + 1}_{nb}_{nt + 2}.dat"
            f" {machine}:/scratch/{username}/cache/6x/scores/",
            shell=True
        )
    if nb < 14:
        subprocess.run(
            f"scp /scratch/{username}/cache/6x/scores/{na}_{nb + 1}_{nt + 2}.dat"
            f" {machine}:/scratch/{username}/cache/6x/scores/",
            shell=True
        )
    if nt > 0:
        subprocess.run(
            f"scp /scratch/{username}/cache/6x/scores/{na}_{nb}_{nt - 1}.dat"
            f" {machine}:/scratch/{username}/cache/6x/scores/",
            shell=True
        )

    subprocess.run(f"ssh {machine} '~/programming/yatzy_solver/solver "
                   f"compute-strat-6x {na} {nb} {nt}'",
                   shell=True)

    subprocess.run(f"scp {machine}:/scratch/{username}/cache/6x/scores/"
                   f"{na}_{nb}_{nt}.dat "
                   f"/scratch/{username}/cache/6x/scores/",
                   shell=True)

    subprocess.run(f"scp {machine}:/scratch/{username}/cache/6x/strats/"
                   f"{na}_{nb}_{nt}.dat "
                   f"/scratch/{username}/cache/6x/strats/",
                   shell=True)


def is_done(na, nb, nt):
    return os.path.isfile(
        f"/scratch/{username}/cache/6x/scores/{na}_{nb}_{nt}.dat"
    )


def is_ready(na, nb, nt):
    if is_done(na, nb, nt):
        return False

    if not (na == 6 or os.path.isfile(
        f"/scratch/{username}/cache/6x/scores/{na + 1}_{nb}_{nt + 2}.dat"
    )):
        return False

    if not (nb == 14 or os.path.isfile(
        f"/scratch/{username}/cache/6x/scores/{na}_{nb + 1}_{nt + 2}.dat"
    )):
        return False

    if not (nt == 0 or os.path.isfile(
        f"/scratch/{username}/cache/6x/scores/{na}_{nb}_{nt - 1}.dat"
    )):
        return False

    return True


to_be_solved = []

for na in reversed(range(7)):
    for nb in reversed(range(15)):
        nt_max = (na + nb) * 2 + 2
        for nt in range(nt_max + 1):
            if not is_done(na, nb, nt):
                to_be_solved.append((na, nb, nt))


def get_solvable():
    for i in range(len(to_be_solved)):
        na, nb, nt = to_be_solved[i]
        if is_ready(na, nb, nt):
            to_be_solved.pop(i)
            return (na, nb, nt)


def get_allowed_machines():
    with open("distributed/machines.txt") as f:
        return [l.strip() for l in f]


running_machines = []


def get_available_machine():
    with open("distributed/machines.txt") as f:
        for l in f:
            machine = l.strip()
            if machine not in running_machines:
                return machine


def is_any_solvable():
    for i in range(len(to_be_solved)):
        na, nb, nt = to_be_solved[i]
        if is_ready(na, nb, nt):
            return True
    return False


running_tasks = []

while not os.path.isfile("distributed/stop") and len(to_be_solved) != 0:
    while True:
        machine = get_available_machine()
        if machine is None:
            break
        state = get_solvable()
        if state is None:
            break

        na, nb, nt = state

        print(f"Running {na} {nb} {nt} on {machine}")

        running_machines.append(machine)

        if machine == "here":
            t = threading.Thread(target=run_here, args=(na, nb, nt,))
        else:
            t = threading.Thread(
                target=run_remote, args=(machine, na, nb, nt,))

        t.start()
        running_tasks.append((machine, t, time.time()))

    while not (is_any_solvable() and get_available_machine() is not None):
        time.sleep(1)
        for i in range(len(running_tasks)):
            machine, t, tm = running_tasks[i]
            if not t.is_alive():
                tt = time.time() - tm
                print(f"Job on {machine} is done in {tt:.2f} s!")
                if machine in get_allowed_machines():
                    running_machines.remove(machine)
                running_tasks.pop(i)
                break
