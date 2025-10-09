namespace bloom_filter
{
    using System.Security.Cryptography;
    using System.Text;

    enum Algo
    {
        Sha256,
        Sha384,
        Sha512
    }

    class Hashing
    {
        //hash function takes first numbers of shaxxx function and represents it as integer 
        public static int Sha(string source, Algo algo, int take = 9)
        {
            HashAlgorithm ha = algo switch
            {
                Algo.Sha256 => SHA256.Create(),
                Algo.Sha384 => SHA384.Create(),
                Algo.Sha512 => SHA512.Create(),
                _ => throw new Exception("Unsupported algo")
            };

            using (ha)
            {
                byte[] sourceBytes = Encoding.UTF8.GetBytes(source);
                byte[] hashBytes = ha.ComputeHash(sourceBytes);
                string hash = BitConverter.ToString(hashBytes).Replace("-", String.Empty);

                return int.Parse(string.Join("", hash.Where(x => Char.IsDigit(x)).Take(take).ToList()));
            }
        }
    }
}