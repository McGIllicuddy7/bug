#pragma once
#include <stdlib.h>
#include <functional>
#include <format>
#include <print>
constexpr size_t ARENA_SIZE = 4096*2;
struct DeferNode{
    std::function<void ()> m_to_call;
    DeferNode * m_next;
};
class Arena{
    char * m_base;
    char * m_next;
    char * m_end;
    Arena * m_next_arena;
    DeferNode * m_at_eol;
    public:
    inline Arena() noexcept{
        m_base = new char[ARENA_SIZE];
        m_next = m_base;
        m_end = m_base+ARENA_SIZE;
        m_at_eol = 0;
        m_next_arena = 0;
    }
    inline Arena(size_t size) noexcept{
        if (size%16 != 0){
            size += 16*size-16;
        }
        if (size<1024){
            size = 1024;
        }
        m_base = new char[size];
        m_next = m_base;
        m_end = m_base+size;
        m_at_eol = 0;
        m_next_arena = 0;
    }
    inline void deinit() noexcept{
        DeferNode * next = m_at_eol;
        while(next){
            next->m_to_call();
            next = next->m_next;
        }
        delete[] m_base;
        delete m_next_arena;
        m_base = 0;
        m_next = 0;
        m_end =0;
        m_at_eol = 0;
        m_next_arena =0;
    }
    inline ~Arena(){
        deinit();
    }
    inline void * alloc(size_t size) noexcept{
        if(size%16 != 0){
            size += 16-size%16;
        }
        if(m_next+size>m_end){
            if (!m_next_arena){
                if(size<4096){
                    size = 4096;
                }
                m_next_arena = new Arena(size);
            }
            return m_next_arena->alloc(size);
        }
        void * out = m_next;
        memset(out, 0, size);
        m_next += size;
        return out;
    }
    template<typename T> T* alloc() noexcept{
        return (T*)alloc(sizeof(T));
    }
    template<typename T> T* alloc(size_t count) noexcept{
        return (T*)alloc(sizeof(T)*count);
    }
    inline void defer(std::function<void()> &&to_call) noexcept{
        DeferNode * prev = m_at_eol;
        DeferNode * next = alloc<DeferNode>();
        next->m_to_call =to_call;
        next->m_next = prev;
        m_at_eol = next;
    };
};
#define DEFER(arena, function, ...){(arena).defer([__VA_ARGS__](){function;});}
