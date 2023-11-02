function range_iter(state)
    if state.cur < state.max then
        state.cur = state.cur + 1
        return state.cur, state.cur * state.cur
    end
end

function range(max)
    return range_iter, {cur = 0, max = max}
end

-- 使用迭代器
for key, value in range(5) do
    print(value)
end

function generic_pairs_iter(state, key)
    local next_key, next_value = next(state.table, key)
    if next_key ~= nil then
        return next_key, next_value
    end -- if there's no next key, Lua will stop the iteration
end

function generic_pairs(t)
    return generic_pairs_iter, {table = t}, nil
end

-- 使用迭代器
for key, value in generic_pairs({a = 1, b = 2, c = 3}) do
    print(key, value)
end
