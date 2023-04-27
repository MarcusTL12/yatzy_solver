using StatsPlots

yz = "target/release/libyatzy_solver.so"

function find_dist_5(n)
    data = zeros(Int32, n)

    ccall((:extern_simulate_n_5, yz), Cvoid, (Ptr{Int32}, Int), data, n)

    data
end
