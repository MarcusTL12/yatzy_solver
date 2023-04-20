
function create_all_combs(n, comb=Int[], all_combs=Vector{Int}[])
    if n == 0
        push!(all_combs, sort(comb))
    else
        for i in 1:6
            push!(comb, i)
            create_all_combs(n - 1, comb, all_combs)
            pop!(comb)
        end
    end

    all_combs
end

function create_unique_combs(n)
    unique!(create_all_combs(n))
end

function create_lookup(n)
    all_combs = create_all_combs(n)

    all_counts = Dict{Vector{Int},Int}()

    for c in all_combs
        all_counts[c] = get(all_counts, c, 0) + 1
    end

    sort!([(k, v) for (k, v) in all_counts])
end
