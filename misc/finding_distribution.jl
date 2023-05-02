using StatsPlots
using Statistics

yz = "target/release/libyatzy_solver.so"

function find_dist_5(n)
    data = zeros(Int32, n)

    ccall((:extern_simulate_n_5, yz), Cvoid, (Ptr{Int32}, Int), data, n)

    data
end

function find_dist_6(n)
    data = zeros(Int32, n)

    ccall((:extern_simulate_n_6, yz), Cvoid, (Ptr{Int32}, Int), data, n)

    data
end
