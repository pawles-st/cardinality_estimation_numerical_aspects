source("common.R")

# viable datasets

cardinalities <- list(seq(from = 1, to = 2000, by = 1), seq(from = 10000, to = 800000, by = 10000))
mult <- list(10, 100)

# experiments config

precisions <- c(4, 8, 12, 16)
iters <- list(5, 100)

# viable algorithms

algorithms <- c("HLL", "GHLLGeo", "GHLLHar", "GHLLPlus")
algorithms.readable <- c("HyperLogLog", "GumbelHyperLogLog (średnia geometryczna)", "GumbelHyperLogLog (średnia harmoniczna)", "GumbelHyperLogLog+")

# algorithm and dataset type to process

alg.chosen <- 4
data.chosen <- 2

# overwrite the variables with the chosen set

cardinalities <- cardinalities[[data.chosen]]
mult <- mult[[data.chosen]]
iters <- iters[[data.chosen]]

alg <- algorithms[alg.chosen]
alg.readable <- algorithms.readable[alg.chosen]

# create the plots

for (prec in precisions) {

	estimates <- list()
	means <- c()

	for (card in cardinalities) {

		# read all estimates

		alg.estimates <- read.data(algorithms[alg.chosen], prec, card, mult)
		estimates <- c(estimates, list(alg.estimates / card))
		means <- c(means, mean(alg.estimates) / card)
	}

	# create a comparison scatterplot

	png(paste0("scatter_", alg, "_", prec, ".png"), width = 1920, height = 1080)

	err <- unlist(estimates) - 1
	err[err < 0] <- 0
	up.diff <- max(err)
	err <- 1 - unlist(estimates)
	err[err < 0] <- 0
	down.diff <- max(err)
	
	par(mar = c(6, 6, 4, 2))
	down.diff

	plot(x = rep(cardinalities, each = iters),
		y = unlist(estimates),
		main = paste0("Wykres punktowy dla algorytmu ", alg.readable, " (k = ", 2^prec, ")"),
		xlab = "",
		ylab = "",
		ylim = c(1 - min(1.5 * down.diff, 0.5), 1 + min(1.5 * up.diff, 0.5)),
		col = rep(rep("green", each = iters), times = length(cardinalities)),
		pch = 16,
		cex.main = 3,
		cex.axis = 2.5
	)
	
	title(xlab = "Liczba unikalnych elementów", line = 3.5, cex.lab = 3)
	title(ylab = "Estymacja / Liczba unikalnych elementów", line = 3.5, cex.lab = 3)

	# mark the means

	#points(x = cardinalities,
		#y = means,
		#col = rep("red", times = length(cardinalities)),
	#)

	# mark the ideal result with a line

	abline(h = 1, lwd = 2)

	# add a legend

	dev.off()
}
