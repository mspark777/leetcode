
SELECT score, dense_rank() over(ORDER BY Scores.score DESC) AS rank FROM Scores
