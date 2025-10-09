class BloomFilter
{
    public BloomFilter(List<Func<string, int>> hashingFunctions, int bitArraySize = 64)
    {
        this.BIT_ARRAY_SIZE = bitArraySize;
        this.bitArray = new bool[bitArraySize];
        this.hashingFunctions = hashingFunctions;
    }

    readonly int BIT_ARRAY_SIZE;
    List<Func<string, int>> hashingFunctions;
    bool[] bitArray;

    public IEnumerable<int> Insert(string input)
    {
        foreach (var hf in hashingFunctions){
            var controlBit = hf.Invoke(input) % BIT_ARRAY_SIZE;
            bitArray[controlBit] = true;
            yield return controlBit;
        }
    }

    public (bool result, List<int> checkedIndexes) Test(string input)
    {
        var checkedIndexes = new List<int>();

        foreach (var hf in hashingFunctions)
        {
            var checkingIndex = hf.Invoke(input) % BIT_ARRAY_SIZE;
            var controlBit = bitArray[checkingIndex];
            checkedIndexes.Add(checkingIndex);

            if (!controlBit)
                return (false, checkedIndexes);
        }

        return (true, checkedIndexes);
    }

    public bool[] DumpBitArray => bitArray;
}
