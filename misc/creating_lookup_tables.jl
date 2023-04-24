
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

# Compacing above the line:

function create_all_line_combs_5()
    all_combs = Pair{NTuple{6,Bool},Int}[]

    for i1 in -1:5, i2 in -1:5, i3 in -1:5, i4 in -1:5, i5 in -1:5, i6 in -1:5
        x = (i1, i2, i3, i4, i5, i6)
        xb = x .>= 0
        xv = x .* xb
        xs = xv .* (1:6)
        s = min(sum(xs), 63)
        push!(all_combs, xb => s)
    end

    unique!(sort!(all_combs))
end

function create_all_line_combs_6()
    all_combs = Pair{NTuple{6,Bool},Int}[]

    for i1 in -1:6, i2 in -1:6, i3 in -1:6, i4 in -1:6, i5 in -1:6, i6 in -1:6
        x = (i1, i2, i3, i4, i5, i6)
        xb = x .>= 0
        xv = x .* xb
        xs = xv .* (1:6)
        s = min(sum(xs), 84)
        push!(all_combs, xb => s)
    end

    unique!(sort!(all_combs))
end

function partition_into_levels(all_combs)
    T = Tuple{Int,Vector{Int}}
    levels = (T[], T[], T[], T[], T[], T[], T[])

    for (x, s) in all_combs
        n = sum(x)
        push!(levels[n+1], (s, collect(x)))
    end

    sort!.(levels)
end

function create_all_line_combs_5_full()
    all_combs = []

    for i1 in -1:5, i2 in -1:5, i3 in -1:5, i4 in -1:5, i5 in -1:5, i6 in -1:5
        x = (i1, i2, i3, i4, i5, i6)
        xb = x .>= 0
        xv = x .* xb
        xs = xv .* (1:6)
        s = min(sum(xs), 63)
        if sum(xb) == 6
            push!(all_combs, xb => s)
        end
    end

    unique!(sort!(all_combs))
end

function count_n_filled(all_combs)
    all_counts = Dict{Int,Int}()

    for (c, _) in all_combs
        n_filled = sum(c)
        all_counts[n_filled] = get(all_counts, n_filled, 0) + 1
    end

    sort!(collect(all_counts))
end

function make_counts_5()
    above_counts = count_n_filled(create_all_line_combs_5())

    below_counts = [i => binomial(9, i) for i in 0:9]

    all_counts = zeros(Int, 16)

    for (na, ac) in above_counts, (nb, bc) in below_counts
        n = na + nb
        all_counts[n+1] += ac * bc
    end

    all_counts
end

function make_counts_6()
    above_counts = count_n_filled(create_all_line_combs_6())

    below_counts = [i => binomial(14, i) for i in 0:14]

    all_counts = zeros(Int, 21)

    for (na, ac) in above_counts, (nb, bc) in below_counts
        n = na + nb
        all_counts[n+1] += ac * bc
    end

    all_counts
end

function make_naive_counts_5()
    [64 * binomial(15, i) for i in 0:15]
end

function make_naive_counts_6()
    [85 * binomial(20, i) for i in 0:20]
end

# Finding size constraints for matrix layers

function find_max_size(M)
    x = 0

    for j in 1:size(M, 2)-1
        for k in axes(M, 1)
            y = 0

            for i in 1:k
                y += M[i, j+1]
            end

            for i in k:size(M, 1)
                y += M[i, j]
            end

            x = max(x, y)
        end
    end

    x
end

function find_ram(A, B, dn)
    M = reverse!(A * B')

    colby = find_max_size(M)
    rowby = find_max_size(M')

    if colby < rowby
        println("Column")
    else
        println("Row")
    end

    s = min(colby, rowby)

    s * dn * 5 / 1024^3
end

function find_storage(A, B, dn)
    sum(A * B') * 3 * dn * 5 / 1024^3
end

function find_ram_5()
    A = [1, 36, 396, 988, 926, 383, 64]
    B = [1, 9, 36, 84, 126, 126, 84, 36, 9, 1]
    dn = 252

    find_ram(A, B, dn)
end

function find_ram_6()
    A = [1, 42, 483, 1201, 1189, 509, 85]
    B = [1, 14, 91, 364, 1001, 2002, 3003, 3432,
        3003, 2002, 1001, 364, 91, 14, 1]
    dn = 462

    find_ram(A, B, dn)
end

function find_storage_5()
    A = [1, 36, 396, 988, 926, 383, 64]
    B = [1, 9, 36, 84, 126, 126, 84, 36, 9, 1]
    dn = 252

    find_storage(A, B, dn)
end

function find_storage_6()
    A = [1, 42, 483, 1201, 1189, 509, 85]
    B = [1, 14, 91, 364, 1001, 2002, 3003, 3432,
        3003, 2002, 1001, 364, 91, 14, 1]
    dn = 462

    find_storage(A, B, dn)
end
