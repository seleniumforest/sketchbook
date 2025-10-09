namespace bloom_filter
{
    internal class Program
    {
        static void Main(string[] args)
        {
            var funcs = new List<Func<string, int>>(){
                (source) => Hashing.Sha(source, Algo.Sha256),
                (source) => Hashing.Sha(source, Algo.Sha384),
                (source) => Hashing.Sha(source, Algo.Sha512),              
            };

            var filter = new BloomFilter(funcs);
            PrintInsertedBits(filter.Insert("Alpaca"));
            PrintBitArray(filter);
            PrintInsertedBits(filter.Insert("Llama"));
            PrintBitArray(filter);
            PrintInsertedBits(filter.Insert("Rabbit"));
            PrintBitArray(filter);

            Console.WriteLine();

            TestAndPrint(filter, "Alpaca");
            TestAndPrint(filter, "Llama");
            TestAndPrint(filter, "Bear");
        }

        static void TestAndPrint(BloomFilter filter, string animal)
        {
            var result = filter.Test(animal);
            Console.WriteLine($"{animal} {(result.result ? "probably exist" : "100% not exist")}. Checked Bits:{string.Join(",", result.checkedIndexes)}");
        }

        static void PrintInsertedBits(IEnumerable<int> ins) =>
            Console.WriteLine($"Inserted bits: {string.Join(",", ins)}");

        static void PrintBitArray(BloomFilter filter) =>
            Console.WriteLine($"Current bit array {string.Join("", filter.DumpBitArray.Select(x => x ? "1" : "0"))}");
    }
}
