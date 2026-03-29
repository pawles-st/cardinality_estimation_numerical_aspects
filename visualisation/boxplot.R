source("common.R")

# datasets config

card <- 500000
mult <- 100
precisions <- c(4, 8, 12, 16)
iters <- 100

# algorithms to analyse

algorithms <- c("HLL", "GHLLGeo", "GHLLHar", "GHLLPlus")
algorithms.readable <- c("HyperLogLog", "GumbelHyperLogLog (średnia geometryczna)", "GumbelHyperLogLog (średnia harmoniczna)", "GumbelHyperLogLog+")
no.algorithms <- length(algorithms)

# create the plots

for (prec in precisions) {

	estimates <- list()

	# read all estimates

	for (i in 1:no.algorithms) {
		alg.estimates <- read.data(algorithms[i], prec, card, mult)
		estimates <- c(estimates, list(alg.estimates / card))
	}

	# create a comparison boxplot

	png(paste0("boxplot_", prec, ".png"), width = 1920, height = 1080)
	
	par(mar = c(6, 6, 4, 2))

	boxplot(estimates,
		main = paste0("Wykres pudełkowy dla algorytmów (n = ", format(card, scientific = FALSE), ", k = ", 2^prec , ")"),
		xlab = "",
		ylab = "",
		names = algorithms,
		col = rainbow(no.algorithms),
		cex.main = 3,
		cex.axis = 2.5
	)
	
	title(xlab = "Algorytm", line = 3.5, cex.lab = 3)
	title(ylab = "Estymacja / Liczba unikalnych elementów", line = 3.5, cex.lab = 3)

	# mark the means

	means <- lapply(estimates, mean)
	points(unlist(means), pch = 3, cex = 2, lwd = 2)

	# mark the cardinality with a line

	abline(h = 1, lwd = 2)

	# add a legend

	legend("topleft",
	       legend = algorithms.readable,
	       fill = rainbow(no.algorithms),
	       bty = "n",
	       cex = 2,
	       bg = "white",
	       box.lwd = 1,
	)

	dev.off()
}
