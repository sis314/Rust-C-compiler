void main()
{
    int i = 5;
    for (int j = 0; j <= i * 2; j++)
    {
        if (j == 3)
        {
            return;
        }
        else
        {
            j = j + 2;
        }
    }
}