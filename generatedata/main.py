import numpy as np

def generate_large_data_chunked(filename='large_data.bin', total_size_mb=1024, chunk_size_mb=100, dtype=np.uint8):
    # Number of bytes per value, based on the chosen data type
    bytes_per_value = np.dtype(dtype).itemsize
    # Number of values needed to achieve the specified total size in megabytes
    num_values_total = (total_size_mb * 1024**2) // bytes_per_value
    # Number of values per chunk
    num_values_per_chunk = (chunk_size_mb * 1024**2) // bytes_per_value
    
    # Open the file for writing
    with open(filename, 'wb') as f:
        # Generate and write the chunks
        for start in range(0, num_values_total, num_values_per_chunk):
            # Calculate the actual chunk size (it may be smaller at the end)
            actual_chunk_size = min(num_values_per_chunk, num_values_total - start)
            # Generate random values for this chunk
            if np.issubdtype(dtype, np.integer):
                data = np.random.randint(np.iinfo(dtype).min, np.iinfo(dtype).max + 1, size=actual_chunk_size, dtype=dtype)
            else:
                data = np.random.random(size=actual_chunk_size).astype(dtype)
            # Write the chunk to the file
            data.tofile(f)

# Example usage to generate 1 GB of data in 100 MB chunks
generate_large_data_chunked(filename='/home/dertuchi/data/int.bin', total_size_mb=1024, chunk_size_mb=100, dtype=np.uint8)
