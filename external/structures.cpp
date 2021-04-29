#include <iostream>
#include <memory>
#include <string>
#include <vector>
#include <cstdint>

extern "C" typedef void(Drop)(void *);

template <int N>
struct BlobData
{
    // N bytes
    uint8_t data[N];
};

template <typename LHS, typename RHS>
union UnionOf
{
    LHS as_lhs;
    RHS as_rhs;

    ~UnionOf() {}
};

template <typename Value>
union Blob
{
    typedef Value value_t;
    typedef BlobData<sizeof(Value)> blob_t;

    _Static_assert(sizeof(value_t) == sizeof(blob_t));

    value_t as_value;
    blob_t as_blob;

    ~Blob() {}
};

// Ptr<T>
extern "C"
{
    typedef std::unique_ptr<int> Ptr;
    typedef Blob<Ptr> PTR_BLOB;

    PTR_BLOB::blob_t lib_ruby_parser_containers_make_ptr_blob(void *ptr)
    {
        PTR_BLOB u = {.as_value = std::unique_ptr<int>((int *)ptr)};
        PTR_BLOB::blob_t result = u.as_blob;
        u.as_value.release(); // prevent running destructor
        return result;
    }

    void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB::blob_t blob, Drop drop)
    {
        PTR_BLOB u = {.as_blob = blob};
        void *raw = u.as_value.release();
        drop(raw);
    }

    void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB::blob_t blob)
    {
        PTR_BLOB u = {.as_blob = blob};
        return u.as_value.release();
    }

    PTR_BLOB::blob_t lib_ruby_parser_containers_null_ptr_blob()
    {
        PTR_BLOB u = {.as_value = std::unique_ptr<int>(nullptr)};
        return u.as_blob;
    }
}

// MaybePtr<T>
extern "C"
{
    typedef std::unique_ptr<int> MaybePtr;
    typedef Blob<MaybePtr> MAYBE_PTR_BLOB;

    MAYBE_PTR_BLOB::blob_t lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr)
    {
        MAYBE_PTR_BLOB u = {.as_value = std::unique_ptr<int>((int *)ptr)};
        MAYBE_PTR_BLOB::blob_t result = u.as_blob;
        u.as_value.release(); // prevent running destructor
        return result;
    }

    void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB::blob_t blob, Drop drop)
    {
        MAYBE_PTR_BLOB u = {.as_blob = blob};
        void *raw = u.as_value.release();
        drop(raw);
    }

    void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(MAYBE_PTR_BLOB::blob_t blob)
    {
        MAYBE_PTR_BLOB u = {.as_blob = blob};
        return u.as_value.get();
    }

    MAYBE_PTR_BLOB::blob_t lib_ruby_parser_containers_null_maybe_ptr_blob()
    {
        MAYBE_PTR_BLOB u = {.as_value = std::unique_ptr<int>(nullptr)};
        return u.as_blob;
    }
}

// List<T>

template <typename Out>
std::vector<int> as_generic_vector(std::vector<Out> vec)
{
    UnionOf<std::vector<int>, std::vector<Out>> u = {.as_rhs = vec};
    return u.as_lhs;
}

extern "C"
{
    typedef std::vector<int> List;
    typedef Blob<List> LIST_BLOB;

    typedef BlobData<192> Node;

    LIST_BLOB::blob_t lib_ruby_parser_containers_make_node_list_blob(Node *ptr, uint64_t size)
    {
        UnionOf<List, std::vector<Node>> list_u = {.as_rhs = std::vector<Node>(ptr, ptr + size)};
        LIST_BLOB u = {.as_value = std::move(list_u.as_lhs)};
        return u.as_blob;
    }

    void lib_ruby_parser_containers_free_node_list_blob(LIST_BLOB::blob_t blob, Drop drop_in_place)
    {
        LIST_BLOB u = {.as_blob = blob};
        UnionOf<List, std::vector<Node>> list_u = {.as_lhs = u.as_value};
        std::vector<Node> vec = std::move(list_u.as_rhs);
        for (size_t i = 0; i < vec.size(); i++)
        {
            drop_in_place(&vec.data()[i]);
        }
    }
}
