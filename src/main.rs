// https://en.wikipedia.org/wiki/Glicko_rating_system
// https://csgo-ranks.com/elo/
// http://www.glicko.net/glicko/glicko2.pdf

// aims for project:
// a command line tool for a persistent mmr ranking system
// where you can enter teams of 5v5, and calculate elo change in all of them
// tick tack toe mmr
// extention: vizulize changes

// every player in glicko2 has
// -- a rating, r,
// -- a rating devation, RD
// -- a rating volitility, sigma
// volitility measure, sigma, indicates the degree of expected fluctiaton in a player's rating
// high when a player has erratic performances (e.g. strong results after a period of stability)
// low when the player performs at a consistent level

// can put player's strength in an interval, rather than a rating, to report 95% confidence interval
// lowest val: rating - 2*RD
// highest val: rating + 2*RD
// e.g. rating 1850, RD 50, interval would be 1750->1950
// we would then say we're 95% confident about a
// player's strength being in a small interval of values.
// the volitility measure does not appear in this interval

// to apply the rating algorithm, we treat a collection of games with a "rating period"
// players would have ratings, RD's, and volitilities at the beginning of the "rating period"
// game outcomes would be observed, and then updated ratings,
// RD's and volitilities would be computed at the end of the rating period
// (which would then be used as the pre-period information for the subsequent rating period)

// the glicko-2 system works best when the number of games in a rating period is moderate to large,
// say an average of at least 10-15 games per player in a rating period
// the length of time for a rating period is at the discretion of the administrator

// the rating scale of glicko-2 is different from that of the original glicko system
// however, it is easy to go back and forth between the two scales.

// -- glicko 2 --

// step 1: determine a rating and RD for each player at the onset of the rating period.
// The system constant, r, which constrains the change in volatility over time,
// needs to be set prior to application of the system.
// reasonable choices are between 0.3 and 1.2, though the system should be tested to decide which
// value results in greatest predictive accuracy.

// smaller values of r prevent the volatility measures from changing by large amounts,
// which in turn prevent enormous rating in changes based on very improbably results
// if the application of the glicko2 is expected to involve extremely improbably
// collections of game outcomes, then r should be set to a small value,
// even as small as say, r = 0.2.

// (a) if the player is unrated, set the rating to 1500, and the RD to 350.
// set the players rv to 0.06 (depends on particular application.)
// (b) otherwise, use the player's most recent rating, RD, and volatility sigma.

// step 2: for each player
//convert the ratings and RD on to the glicko-2 scale.

// mu = (r - 1500)/173.7178
// phi = RD/173.7178
// sigma, volitility, does not change

// we now want to update the rating of a player
// with Glicko-2 rating (mu), rating deviation (phi), and volitility (sigma).
// we play against X opponents
// with ratings (mu1... muX)
// with rating devations (phi1... phiX)
// with scores (s1... sX)... 0 for loss, 0.5 for draw, 1 for win
// the opponents volatilities are not relevant in the calcuations.

// step 3:
// compute the quantity, v.
// This is the estimated variance of the team's/player's rating based only on game outcomes.

// for j = 1, to m
// v = 1 / sum( g(phiJ)*g(phiJ) * e(mu, muJ, phiJ) )  .. set (1, e(mu, muJ, phiJ))
// g(phi) = 1.0 / sqrt(1 + 3*phi*phi / pi*pi)
// e(mu, muJ, phiJ) = 1.0 / 1.0 + exp( -g(phiJ) *  mu - muJ))

// step 4:
// compute the quantity, delta
// the estimated improvement in rating by comparing the pre-period rating
// to the performance rating based only on game outcomes.

// for j = 1, to m
// delta = v * sum... g(phiJ) .. set (sJ - e(mu, muJ, phiJ))

// (TODO) step 5:
// determine the new value, phi_new, of the volatility
// this computation requires iteration

fn main() {
    println!("Hello, world!");
}
