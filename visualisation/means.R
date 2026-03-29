source("common.R")

# viable datasets

cardinalities <- list(seq(from = 1, to = 2000, by = 1), seq(from = 10000, to = 800000, by = 10000))
mult <- list(10, 100)

# experiments config

precisions <- c(4, 8, 12, 16)
iters <- 100

# algorithms

algorithms <- c("HLL", "GHLLGeo", "GHLLHar", "GHLLPlus")
algorithms.readable <- c("HyperLogLog", "GumbelHyperLogLog (średnia geometryczna)", "GumbelHyperLogLog (średnia harmoniczna)", "GumbelHyperLogLog+")
no.algorithms <- length(algorithms)

# dataset type to process

data.chosen <- 2

# overwrite the variables with the chosen set

cardinalities <- cardinalities[[data.chosen]]
mult <- mult[[data.chosen]]

# create the plots

for (prec in precisions) {

	means <- c()

	for (card in cardinalities) {

		# read all estimates

		for (i in 1:no.algorithms) {
			alg.estimates <- read.data(algorithms[i], prec, card, mult)
			means <- c(means, mean(alg.estimates) / card)
		}
	}
	max(means)

	# create a comparison lineplot

	png(paste0("means_", prec, ".png"), width = 1920, height = 1080)

	err <- means - 1
	err[err < 0] <- 0
	up.diff <- abs(max(err))
	err <- 1 - means
	err[err < 0] <- 0
	down.diff <- abs(max(err))
	
	par(mar = c(6, 6, 4, 2))

	plot(x = cardinalities,
		y = rep(1, length(cardinalities)),
		main = paste0("Porównanie jakości empirycznych wartości oczekiwanych algorytmów, (k = ", 2 ^ prec, ")"),
		xlab = "",
		ylab = "",
		ylim = c(1 - min(down.diff, 0.5), 1 + min(up.diff, 0.5)),
		type = "n",
		cex.main = 3,
		cex.axis = 2.5,
	)

	title(xlab = "Liczba unikalnych elementów", line = 3.5, cex.lab = 3)
	title(ylab = "Średnia arytmetyczna estymacji / Liczba unikalnych elementów", line = 3.5, cex.lab = 3)

	# mark the means

	if (data.chosen == 1) {
		mark = points
	} else {
		mark = lines
	}

	cols <- rainbow(no.algorithms)
	for (i in 1:no.algorithms)
		mark(x = cardinalities,
			y = means[seq(i, length(means), by = no.algorithms)],
			col = cols[i],
			lwd = 1.5,
			pch = 21,
			bg = cols[i],
	)

	# mark the ideal result with a line

	abline(h = 1, lwd = 2)

	# add a legend

	legend("topright",
	       legend = algorithms.readable,
	       fill = rainbow(no.algorithms),
	       bty = "n",
	       cex = 2,
	       bg = "white",
	       box.lwd = 1,
	)

	dev.off()
}
