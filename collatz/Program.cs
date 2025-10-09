using Plotly.NET;

const int start = 10;
const int count = 10;

IEnumerable<int> getSequence(int num)
{
    while (num != 1)
    {
        num = num % 2 == 0 ? num / 2 : num * 3 + 1;
        yield return num;
    }
}

Chart.Combine(
    Enumerable.Range(start, count)
    .ToList()
    .Select(num =>
    {
        var seq = getSequence(num);
        var chart = Chart2D.Chart.Line<int, int, int>(
            x: Enumerable.Range(0, seq.ToList().Count),
            y: seq,
            ShowLegend: true,
            Name: $"Number {num}");
        return chart;
    })
)
.WithSize(1300, 700)
.Show();